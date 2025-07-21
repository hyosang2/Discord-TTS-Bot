use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;
use serde::Serialize;
use tracing::{error, info, warn};
use whatlang::{detect, Lang};

use crate::structs::Data;

/// Voice clip metadata
#[derive(Debug, Clone)]
pub struct VoiceClip {
    pub voice_name: String,
    pub language: String,
    pub file_path: PathBuf,
}

/// Voice collection for a specific voice name
#[derive(Debug, Clone)]
pub struct Voice {
    pub name: String,
    pub clips: HashMap<String, PathBuf>, // language -> file path
}

/// XTTS character limit (conservative estimate based on timeouts observed)
const XTTS_CHAR_LIMIT: usize = 250;

/// Native XTTS API request structure for /tts_to_audio/ endpoint
#[derive(Debug, Serialize)]
struct NativeXTTSRequest {
    text: String,
    speaker_wav: String,
    language: String,
}

/// Cache for voice clips
pub type VoiceCache = Arc<RwLock<HashMap<String, Voice>>>;

/// Initialize the voice cache from the voice clips directory
pub async fn init_voice_cache(voice_clips_path: &Path) -> Result<VoiceCache> {
    let mut voices: HashMap<String, Voice> = HashMap::new();
    
    // Check if directory exists
    if !voice_clips_path.exists() {
        warn!("Voice clips directory does not exist: {:?}", voice_clips_path);
        // Create default voice entry
        voices.insert(
            "default".to_string(),
            Voice {
                name: "default".to_string(),
                clips: HashMap::new(),
            },
        );
        return Ok(Arc::new(RwLock::new(voices)));
    }
    
    // Scan directory for voice folders
    let entries = std::fs::read_dir(voice_clips_path)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let voice_name = match path.file_name().and_then(|n| n.to_str()) {
                Some(name) => name.to_string(),
                None => continue,
            };
            
            let mut clips = HashMap::new();
            
            // Scan voice directory for language clips
            let voice_entries = std::fs::read_dir(&path)?;
            
            for voice_entry in voice_entries {
                let voice_entry = voice_entry?;
                let clip_path = voice_entry.path();
                
                if clip_path.is_file() && clip_path.extension().map_or(false, |ext| ext == "wav") {
                    if let Some(file_stem) = clip_path.file_stem().and_then(|s| s.to_str()) {
                        // File should be named like "en.wav" or "es.wav"
                        clips.insert(file_stem.to_string(), clip_path);
                    }
                }
            }
            
            if !clips.is_empty() {
                voices.insert(
                    voice_name.clone(),
                    Voice {
                        name: voice_name,
                        clips,
                    },
                );
            }
        }
    }
    
    // Ensure default voice exists
    if !voices.contains_key("default") {
        voices.insert(
            "default".to_string(),
            Voice {
                name: "default".to_string(),
                clips: HashMap::new(),
            },
        );
    }
    
    info!("Loaded {} voices from {:?}", voices.len(), voice_clips_path);
    Ok(Arc::new(RwLock::new(voices)))
}

/// Detect language from text
pub fn detect_language(text: &str) -> Option<String> {
    detect(text).map(|info| {
        match info.lang() {
            Lang::Eng => "en",
            Lang::Spa => "es",
            Lang::Fra => "fr",
            Lang::Deu => "de",
            Lang::Ita => "it",
            Lang::Por => "pt",
            Lang::Pol => "pl",
            Lang::Tur => "tr",
            Lang::Rus => "ru",
            Lang::Nld => "nl",
            Lang::Ces => "cs",
            Lang::Ara => "ar",
            Lang::Cmn => "zh-cn",
            Lang::Jpn => "ja",
            Lang::Hun => "hu",
            Lang::Kor => "ko",
            Lang::Hin => "hi",
            _ => "en", // Default to English for unsupported languages
        }.to_string()
    })
}

/// Split text into chunks that fit within XTTS character limit
fn split_text_for_xtts(text: &str) -> Vec<String> {
    if text.len() <= XTTS_CHAR_LIMIT {
        return vec![text.to_string()];
    }
    
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    
    // Split by sentences first (periods, exclamation marks, question marks)
    let sentence_endings = ['.', '!', '?'];
    let mut sentence_start = 0;
    
    for (i, ch) in text.char_indices() {
        if sentence_endings.contains(&ch) {
            let sentence = &text[sentence_start..=i];
            
            // If adding this sentence would exceed the limit, save current chunk
            if current_chunk.len() + sentence.len() > XTTS_CHAR_LIMIT && !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
                current_chunk.clear();
            }
            
            current_chunk.push_str(sentence);
            sentence_start = i + 1;
            
            // If this single sentence is too long, we need to split by words
            if current_chunk.len() > XTTS_CHAR_LIMIT {
                chunks.push(current_chunk.trim().to_string());
                current_chunk.clear();
            }
        }
    }
    
    // Add remaining text if sentence-based splitting didn't cover everything
    if sentence_start < text.len() {
        let remaining = &text[sentence_start..];
        if current_chunk.len() + remaining.len() > XTTS_CHAR_LIMIT && !current_chunk.is_empty() {
            chunks.push(current_chunk.trim().to_string());
            current_chunk.clear();
        }
        current_chunk.push_str(remaining);
    }
    
    // Add the last chunk if it exists
    if !current_chunk.is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }
    
    // Fallback: if we still have chunks that are too long, split by words
    let mut final_chunks = Vec::new();
    for chunk in chunks {
        if chunk.len() <= XTTS_CHAR_LIMIT {
            final_chunks.push(chunk);
        } else {
            // Split by words as last resort
            let words: Vec<&str> = chunk.split_whitespace().collect();
            let mut word_chunk = String::new();
            
            for word in words {
                if word_chunk.len() + word.len() + 1 > XTTS_CHAR_LIMIT && !word_chunk.is_empty() {
                    final_chunks.push(word_chunk.trim().to_string());
                    word_chunk.clear();
                }
                if !word_chunk.is_empty() {
                    word_chunk.push(' ');
                }
                word_chunk.push_str(word);
            }
            
            if !word_chunk.is_empty() {
                final_chunks.push(word_chunk.trim().to_string());
            }
        }
    }
    
    // Ensure we don't return empty chunks
    final_chunks.into_iter().filter(|chunk| !chunk.trim().is_empty()).collect()
}

/// Get the best voice clip for the given voice and language
pub fn get_voice_clip_path(
    voice_cache: &VoiceCache,
    voice_name: &str,
    detected_language: Option<&str>,
) -> Option<PathBuf> {
    let voices = voice_cache.read();
    
    // Get the voice, fallback to default if not found
    let voice = voices.get(voice_name)
        .or_else(|| voices.get("default"))?;
    
    // Try to find clip in this order:
    // 1. Detected language
    // 2. English
    // 3. Any available clip
    if let Some(lang) = detected_language {
        if let Some(path) = voice.clips.get(lang) {
            return Some(path.clone());
        }
    }
    
    // Fallback to English
    if let Some(path) = voice.clips.get("en") {
        return Some(path.clone());
    }
    
    // Fallback to any available clip
    voice.clips.values().next().cloned()
}

/// Fetch audio from a single text chunk using native XTTS API
async fn fetch_xtts_chunk(
    data: &Data,
    text: &str,
    speaker_wav: &str,
    language: &str,
    _speaking_rate: f32,
    xtts_service_url: &reqwest::Url,
) -> Result<Vec<u8>> {
    // Create request for native XTTS API
    let request = NativeXTTSRequest {
        text: text.to_string(),
        speaker_wav: speaker_wav.to_string(),
        language: language.to_string(),
    };
    
    // Make TTS API request to native /tts_to_audio/ endpoint
    let mut url = xtts_service_url.clone();
    url.set_path("/tts_to_audio/");
    
    let response = data.reqwest
        .post(url)
        .json(&request)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        error!("Native XTTS API error {}: {}", status, error_text);
        return Err(anyhow::anyhow!("Native XTTS API error: {}", status));
    }
    
    // Native XTTS returns raw audio bytes directly
    let audio_bytes = response.bytes().await?.to_vec();
    
    Ok(audio_bytes)
}

/// Fetch audio from XTTS service (with text chunking support)
pub async fn fetch_xtts_audio(
    data: &Data,
    text: &str,
    voice_name: &str,
    speaking_rate: f32,
) -> Result<Option<Vec<u8>>> {
    let Some(xtts_service_url) = &data.config.xtts_service else {
        error!("XTTS service URL not configured");
        return Ok(None);
    };
    
    // Detect language from text
    let detected_language = detect_language(text);
    
    // Get voice clip path
    let voice_clip_path = match get_voice_clip_path(
        &data.xtts_voice_cache,
        voice_name,
        detected_language.as_deref(),
    ) {
        Some(path) => path,
        None => {
            warn!("No voice clip found for voice: {}", voice_name);
            return Ok(None);
        }
    };
    
    // Convert path to relative format for native API (voice_name/language.wav)
    let speaker_wav = format!("{}/{}", voice_name, 
        voice_clip_path.file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("en.wav"));
    
    // Use detected language or default to English
    let language = detected_language.unwrap_or_else(|| "en".to_string());
    
    // Split text into chunks if it's too long
    let text_chunks = split_text_for_xtts(text);
    
    if text_chunks.len() > 1 {
        info!("Splitting long text into {} chunks for XTTS processing", text_chunks.len());
    }
    
    let mut all_audio_data = Vec::new();
    
    // Process each chunk
    for (i, chunk) in text_chunks.iter().enumerate() {
        info!("Processing chunk {}/{}: {} characters", i + 1, text_chunks.len(), chunk.len());
        
        match fetch_xtts_chunk(
            data,
            chunk,
            &speaker_wav,
            &language,
            speaking_rate,
            xtts_service_url,
        ).await {
            Ok(audio_data) => {
                all_audio_data.extend(audio_data);
                
                // Add a small silence between chunks (except for the last one)
                if i < text_chunks.len() - 1 {
                    // Add ~0.3 seconds of silence (rough estimate for 22050 Hz, 16-bit mono)
                    let silence_samples = (22050.0 * 0.3 * 2.0) as usize; // 2 bytes per sample
                    let silence_bytes = vec![0u8; silence_samples];
                    all_audio_data.extend(silence_bytes);
                }
            }
            Err(e) => {
                error!("Failed to process chunk {}: {}", i + 1, e);
                return Err(e);
            }
        }
    }
    
    if all_audio_data.is_empty() {
        warn!("No audio data generated for text: {}", text);
        return Ok(None);
    }
    
    info!("Generated {} bytes of audio for {} chunks", all_audio_data.len(), text_chunks.len());
    Ok(Some(all_audio_data))
}


/// Get list of available voices
pub fn get_available_voices(voice_cache: &VoiceCache) -> Vec<(String, Vec<String>)> {
    let voices = voice_cache.read();
    
    voices.iter()
        .map(|(name, voice)| {
            let languages: Vec<String> = voice.clips.keys().cloned().collect();
            (name.clone(), languages)
        })
        .collect()
}