// SENTINEL AI Prediction Engine Example
// This example demonstrates how to use the AI Prediction Engine

use sentinel::ai::{AIPredictionEngine, Features, Prediction, ThreatType, Action};
use sentinel::error::{SentinelError, Result};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🤖 SENTINEL AI Prediction Engine Example\n");

    // Step 1: Initialize AI Engine
    println!("Step 1: Initializing AI Prediction Engine...");
    let mut ai_engine = AIPredictionEngine::new();
    
    // Load pre-trained model
    ai_engine.load_model("models/threat-predictor.pt").await?;
    println!("✅ AI Engine initialized\n");

    // Step 2: Single Prediction
    println!("Step 2: Making single prediction...");
    let features = create_sample_features();
    
    let prediction = ai_engine.predict(&features).await?;
    println!("✅ Prediction completed:");
    println!("   Threat Type: {:?}", prediction.threat_type);
    println!("   Confidence: {:.2}%", prediction.confidence * 100.0);
    println!("   Risk Score: {:.2}", prediction.risk_score);
    println!("   Recommended Action: {:?}\n", prediction.recommended_action);

    // Step 3: Batch Prediction
    println!("Step 3: Making batch predictions...");
    let features_batch = vec![
        create_benign_features(),
        create_malicious_features(),
        create_suspicious_features(),
    ];
    
    let predictions = ai_engine.predict_batch(&features_batch).await?;
    println!("✅ Batch predictions completed:");
    for (i, pred) in predictions.iter().enumerate() {
        println!("   Prediction {}: {:?} (Confidence: {:.2}%)", 
                 i + 1, pred.threat_type, pred.confidence * 100.0);
    }
    println!();

    // Step 4: Real-time Monitoring
    println!("Step 4: Real-time threat monitoring (5 iterations)...");
    for i in 1..=5 {
        let features = generate_random_features();
        let prediction = ai_engine.predict(&features).await?;
        
        println!("   Iteration {}: {:?} - {:.2}% confidence", 
                 i, prediction.threat_type, prediction.confidence * 100.0);
        
        // Take action if threat detected
        if prediction.risk_score > 0.7 {
            println!("   ⚠️  High risk detected! Taking action...");
            take_action(&prediction.recommended_action).await?;
        }
        
        sleep(Duration::from_millis(500)).await;
    }
    println!();

    // Step 5: Model Information
    println!("Step 5: Model Information");
    let model_info = ai_engine.get_model_info().await?;
    println!("   Model Name: {}", model_info.name);
    println!("   Model Version: {}", model_info.version);
    println!("   Accuracy: {:.2}%", model_info.accuracy * 100.0);
    println!("   False Positive Rate: {:.3}%", model_info.false_positive_rate * 100.0);
    println!("   Training Data Size: {}", model_info.training_data_size);
    println!();

    // Step 6: Cleanup
    println!("Step 6: Cleaning up...");
    ai_engine.cleanup().await?;
    println!("✅ AI Engine cleaned up\n");

    println!("🎉 AI Prediction Engine example completed successfully!");
    
    Ok(())
}

// Create sample features for prediction
fn create_sample_features() -> Features {
    Features {
        process_behavior: vec![0.1, 0.2, 0.3, 0.4, 0.5],
        file_operations: vec![0.6, 0.7, 0.8, 0.9, 1.0],
        network_activity: vec![0.1, 0.2, 0.3, 0.4, 0.5],
        system_calls: vec![0.6, 0.7, 0.8, 0.9, 1.0],
        registry_changes: vec![0.1, 0.2, 0.3, 0.4, 0.5],
        memory_access: vec![0.2, 0.3, 0.4, 0.5, 0.6],
        cpu_usage: vec![0.1, 0.2, 0.3, 0.4, 0.5],
        network_connections: vec![0.3, 0.4, 0.5, 0.6, 0.7],
    }
}

// Create benign features (safe process)
fn create_benign_features() -> Features {
    Features {
        process_behavior: vec![0.1, 0.15, 0.12, 0.13, 0.11],
        file_operations: vec![0.05, 0.08, 0.06, 0.07, 0.05],
        network_activity: vec![0.02, 0.03, 0.02, 0.03, 0.02],
        system_calls: vec![0.1, 0.12, 0.11, 0.13, 0.1],
        registry_changes: vec![0.01, 0.01, 0.01, 0.01, 0.01],
        memory_access: vec![0.05, 0.06, 0.05, 0.07, 0.05],
        cpu_usage: vec![0.02, 0.03, 0.02, 0.03, 0.02],
        network_connections: vec![0.01, 0.02, 0.01, 0.02, 0.01],
    }
}

// Create malicious features (malware behavior)
fn create_malicious_features() -> Features {
    Features {
        process_behavior: vec![0.9, 0.95, 0.92, 0.94, 0.91],
        file_operations: vec![0.85, 0.9, 0.88, 0.89, 0.87],
        network_activity: vec![0.92, 0.95, 0.93, 0.94, 0.92],
        system_calls: vec![0.88, 0.91, 0.89, 0.9, 0.88],
        registry_changes: vec![0.95, 0.97, 0.96, 0.97, 0.95],
        memory_access: vec![0.9, 0.93, 0.91, 0.92, 0.9],
        cpu_usage: vec![0.85, 0.88, 0.86, 0.87, 0.85],
        network_connections: vec![0.87, 0.9, 0.88, 0.89, 0.87],
    }
}

// Create suspicious features (potential threat)
fn create_suspicious_features() -> Features {
    Features {
        process_behavior: vec![0.5, 0.6, 0.55, 0.58, 0.52],
        file_operations: vec![0.4, 0.5, 0.45, 0.48, 0.42],
        network_activity: vec![0.6, 0.7, 0.65, 0.68, 0.62],
        system_calls: vec![0.45, 0.55, 0.5, 0.53, 0.47],
        registry_changes: vec![0.7, 0.75, 0.72, 0.74, 0.71],
        memory_access: vec![0.55, 0.6, 0.57, 0.59, 0.56],
        cpu_usage: vec![0.4, 0.5, 0.45, 0.48, 0.42],
        network_connections: vec![0.5, 0.6, 0.55, 0.58, 0.52],
    }
}

// Generate random features for testing
fn generate_random_features() -> Features {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    Features {
        process_behavior: (0..5).map(|_| rng.gen::<f32>()).collect(),
        file_operations: (0..5).map(|_| rng.gen::<f32>()).collect(),
        network_activity: (0..5).map(|_| rng.gen::<f32>()).collect(),
        system_calls: (0..5).map(|_| rng.gen::<f32>()).collect(),
        registry_changes: (0..5).map(|_| rng.gen::<f32>()).collect(),
        memory_access: (0..5).map(|_| rng.gen::<f32>()).collect(),
        cpu_usage: (0..5).map(|_| rng.gen::<f32>()).collect(),
        network_connections: (0..5).map(|_| rng.gen::<f32>()).collect(),
    }
}

// Take action based on prediction
async fn take_action(action: &Action) -> Result<()> {
    match action {
        Action::Allow => {
            println!("   ✅ Allowing process to continue");
        }
        Action::Monitor => {
            println!("   👁️  Monitoring process");
        }
        Action::Quarantine => {
            println!("   🚫 Quarantining process");
            // Implement quarantine logic
        }
        Action::Block => {
            println!("   🛑 Blocking process");
            // Implement block logic
        }
        Action::Terminate => {
            println!("   💀 Terminating process");
            // Implement terminate logic
        }
    }
    
    Ok(())
}

// Example: Continuous monitoring with threat detection
async fn continuous_monitoring(ai_engine: &mut AIPredictionEngine) -> Result<()> {
    println!("🔄 Starting continuous threat monitoring...\n");
    
    loop {
        // Collect system metrics
        let features = collect_system_metrics();
        
        // Make prediction
        let prediction = ai_engine.predict(&features).await?;
        
        // Check if threat detected
        if prediction.risk_score > 0.5 {
            println!("⚠️  Potential threat detected!");
            println!("   Type: {:?}", prediction.threat_type);
            println!("   Risk Score: {:.2}", prediction.risk_score);
            println!("   Confidence: {:.2}%", prediction.confidence * 100.0);
            
            // Take appropriate action
            if prediction.risk_score > 0.7 {
                take_action(&prediction.recommended_action).await?;
            } else {
                println!("   Continuing to monitor...");
            }
        }
        
        sleep(Duration::from_secs(1)).await;
    }
}

// Collect system metrics for monitoring
fn collect_system_metrics() -> Features {
    // In a real implementation, this would collect actual system metrics
    // For this example, we generate random features
    generate_random_features()
}

// Example: Custom threat classification
async fn classify_custom_process(
    ai_engine: &mut AIPredictionEngine,
    process_name: &str,
    process_pid: u32,
) -> Result<bool> {
    println!("🔍 Analyzing process: {} (PID: {})", process_name, process_pid);
    
    // Collect process-specific features
    let features = collect_process_features(process_name, process_pid);
    
    // Make prediction
    let prediction = ai_engine.predict(&features).await?;
    
    // Print results
    println!("   Threat Type: {:?}", prediction.threat_type);
    println!("   Confidence: {:.2}%", prediction.confidence * 100.0);
    println!("   Risk Score: {:.2}", prediction.risk_score);
    println!("   Is Malicious: {}", prediction.risk_score > 0.5);
    
    Ok(prediction.risk_score > 0.5)
}

// Collect process-specific features
fn collect_process_features(process_name: &str, process_pid: u32) -> Features {
    // In a real implementation, this would collect actual process features
    // For this example, we create sample features based on process name
    if process_name.contains("suspicious") || process_name.contains("malware") {
        create_malicious_features()
    } else if process_name.contains("benign") || process_name.contains("safe") {
        create_benign_features()
    } else {
        generate_random_features()
    }
}

// Example: Performance benchmarking
async fn benchmark_prediction(ai_engine: &mut AIPredictionEngine, iterations: usize) -> Result<()> {
    println!("⚡ Benchmarking prediction performance ({} iterations)...\n", iterations);
    
    let start = std::time::Instant::now();
    let mut total_confidence = 0.0;
    let mut high_risk_count = 0;
    
    for i in 0..iterations {
        let features = generate_random_features();
        let prediction = ai_engine.predict(&features).await?;
        
        total_confidence += prediction.confidence;
        if prediction.risk_score > 0.5 {
            high_risk_count += 1;
        }
        
        if (i + 1) % 100 == 0 {
            println!("   Completed {} predictions...", i + 1);
        }
    }
    
    let duration = start.elapsed();
    let avg_time_per_prediction = duration.as_secs_f64() / iterations as f64;
    let avg_confidence = total_confidence / iterations as f64;
    
    println!("\n📊 Benchmark Results:");
    println!("   Total Iterations: {}", iterations);
    println!("   Total Time: {:.2}s", duration.as_secs_f64());
    println!("   Average Time per Prediction: {:.4}ms", avg_time_per_prediction * 1000.0);
    println!("   Average Confidence: {:.2}%", avg_confidence * 100.0);
    println!("   High Risk Count: {} ({:.2}%)", 
             high_risk_count, (high_risk_count as f64 / iterations as f64) * 100.0);
    println!("   Predictions per Second: {:.2}", 1.0 / avg_time_per_prediction);
    
    Ok(())
}