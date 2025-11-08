//! Performance optimization utilities

use std::time::{Duration, Instant};

/// Performance metrics tracker
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Operation name
    pub name: String,
    /// Duration of operation
    pub duration: Duration,
    /// Bytes processed
    pub bytes: usize,
}

impl PerformanceMetrics {
    /// Create new metrics
    pub fn new(name: String, duration: Duration, bytes: usize) -> Self {
        Self {
            name,
            duration,
            bytes,
        }
    }

    /// Get throughput in MB/s
    pub fn throughput_mbps(&self) -> f64 {
        if self.duration.as_secs_f64() == 0.0 {
            0.0
        } else {
            (self.bytes as f64 / 1_000_000.0) / self.duration.as_secs_f64()
        }
    }

    /// Get duration in milliseconds
    pub fn duration_ms(&self) -> f64 {
        self.duration.as_secs_f64() * 1000.0
    }
}

/// Performance timer for measuring operations
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    /// Create a new timer
    pub fn start(name: &str) -> Self {
        Self {
            start: Instant::now(),
            name: name.to_string(),
        }
    }

    /// Stop timer and get metrics
    pub fn stop(self, bytes: usize) -> PerformanceMetrics {
        let duration = self.start.elapsed();
        PerformanceMetrics::new(self.name, duration, bytes)
    }

    /// Stop timer without bytes
    pub fn stop_no_bytes(self) -> Duration {
        self.start.elapsed()
    }
}

/// Batch processor for efficient processing
pub struct BatchProcessor<T> {
    items: Vec<T>,
    batch_size: usize,
}

impl<T> BatchProcessor<T> {
    /// Create a new batch processor
    pub fn new(batch_size: usize) -> Self {
        Self {
            items: Vec::new(),
            batch_size,
        }
    }

    /// Add item to batch
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    /// Get next batch
    pub fn next_batch(&mut self) -> Option<Vec<T>> {
        if self.items.is_empty() {
            None
        } else if self.items.len() <= self.batch_size {
            Some(self.items.drain(..).collect())
        } else {
            let batch: Vec<T> = self.items.drain(..self.batch_size).collect();
            Some(batch)
        }
    }

    /// Get all remaining items
    pub fn drain(self) -> Vec<T> {
        self.items
    }

    /// Get item count
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_performance_metrics() {
        let duration = Duration::from_millis(100);
        let metrics = PerformanceMetrics::new("test".to_string(), duration, 1_000_000);
        
        assert_eq!(metrics.name, "test");
        assert_eq!(metrics.bytes, 1_000_000);
        assert!(metrics.throughput_mbps() > 0.0);
        assert!(metrics.duration_ms() > 0.0);
    }

    #[test]
    fn test_timer() {
        let timer = Timer::start("operation");
        thread::sleep(Duration::from_millis(10));
        let duration = timer.stop_no_bytes();
        
        assert!(duration.as_millis() >= 10);
    }

    #[test]
    fn test_timer_with_bytes() {
        let timer = Timer::start("operation");
        let metrics = timer.stop(1_000_000);
        
        assert_eq!(metrics.name, "operation");
        assert_eq!(metrics.bytes, 1_000_000);
    }

    #[test]
    fn test_batch_processor() {
        let mut processor = BatchProcessor::new(3);
        processor.add(1);
        processor.add(2);
        processor.add(3);
        processor.add(4);
        
        let batch1 = processor.next_batch();
        assert_eq!(batch1, Some(vec![1, 2, 3]));
        
        let batch2 = processor.next_batch();
        assert_eq!(batch2, Some(vec![4]));
        
        let batch3 = processor.next_batch();
        assert_eq!(batch3, None);
    }

    #[test]
    fn test_batch_processor_drain() {
        let mut processor = BatchProcessor::new(2);
        processor.add(1);
        processor.add(2);
        processor.add(3);
        
        let remaining = processor.drain();
        assert_eq!(remaining, vec![1, 2, 3]);
    }
}
