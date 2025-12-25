//! Template module for common presentation types
//!
//! Provides pre-built presentation structures for common use cases.
//!
//! # Examples
//!
//! ```rust
//! use ppt_rs::templates::{self, ProposalContent};
//!
//! // Create a business proposal
//! let pptx = templates::business_proposal(
//!     "Q4 Budget Proposal",
//!     "Finance Team",
//!     ProposalContent {
//!         executive_summary: vec!["Key insight 1", "Key insight 2"],
//!         problem: vec!["Current challenge"],
//!         solution: vec!["Our approach"],
//!         timeline: vec![("Phase 1", "Week 1-2"), ("Phase 2", "Week 3-4")],
//!         budget: vec![("Item", "$10,000")],
//!         next_steps: vec!["Action 1", "Action 2"],
//!     },
//! ).expect("Failed to create presentation");
//!
//! assert!(!pptx.is_empty());
//! ```

use crate::generator::{SlideContent, SlideLayout, create_pptx_with_content};
use crate::exc::Result;

/// Content for a business proposal presentation
#[derive(Debug, Clone)]
pub struct ProposalContent<'a> {
    pub executive_summary: Vec<&'a str>,
    pub problem: Vec<&'a str>,
    pub solution: Vec<&'a str>,
    pub timeline: Vec<(&'a str, &'a str)>,
    pub budget: Vec<(&'a str, &'a str)>,
    pub next_steps: Vec<&'a str>,
}

impl<'a> Default for ProposalContent<'a> {
    fn default() -> Self {
        Self {
            executive_summary: vec!["Add executive summary points"],
            problem: vec!["Define the problem"],
            solution: vec!["Present your solution"],
            timeline: vec![("Phase 1", "Description")],
            budget: vec![("Item", "Amount")],
            next_steps: vec!["Define next steps"],
        }
    }
}

/// Content for a training material presentation
#[derive(Debug, Clone)]
pub struct TrainingContent<'a> {
    pub objectives: Vec<&'a str>,
    pub modules: Vec<(&'a str, Vec<&'a str>)>,
    pub exercises: Vec<&'a str>,
    pub summary: Vec<&'a str>,
}

impl<'a> Default for TrainingContent<'a> {
    fn default() -> Self {
        Self {
            objectives: vec!["Learning objective 1"],
            modules: vec![("Module 1", vec!["Topic 1", "Topic 2"])],
            exercises: vec!["Practice exercise"],
            summary: vec!["Key takeaway"],
        }
    }
}

/// Content for a status report presentation
#[derive(Debug, Clone)]
pub struct StatusContent<'a> {
    pub summary: Vec<&'a str>,
    pub completed: Vec<&'a str>,
    pub in_progress: Vec<&'a str>,
    pub blocked: Vec<&'a str>,
    pub next_week: Vec<&'a str>,
    pub metrics: Vec<(&'a str, &'a str)>,
}

impl<'a> Default for StatusContent<'a> {
    fn default() -> Self {
        Self {
            summary: vec!["High-level status"],
            completed: vec!["Completed item"],
            in_progress: vec!["In progress item"],
            blocked: vec!["Blocked item"],
            next_week: vec!["Planned item"],
            metrics: vec![("Metric", "Value")],
        }
    }
}

/// Content for a technical documentation presentation
#[derive(Debug, Clone)]
pub struct TechnicalContent<'a> {
    pub overview: Vec<&'a str>,
    pub architecture: Vec<&'a str>,
    pub components: Vec<(&'a str, Vec<&'a str>)>,
    pub api_examples: Vec<(&'a str, &'a str)>,
    pub best_practices: Vec<&'a str>,
}

impl<'a> Default for TechnicalContent<'a> {
    fn default() -> Self {
        Self {
            overview: vec!["System overview"],
            architecture: vec!["Architecture description"],
            components: vec![("Component", vec!["Feature 1"])],
            api_examples: vec![("Method", "Description")],
            best_practices: vec!["Best practice"],
        }
    }
}

/// Create a business proposal presentation
///
/// Structure:
/// 1. Title slide
/// 2. Agenda
/// 3. Executive Summary
/// 4. Problem Statement
/// 5. Proposed Solution
/// 6. Timeline
/// 7. Budget
/// 8. Next Steps
/// 9. Questions/Discussion
pub fn business_proposal<'a>(
    title: &str,
    author: &str,
    content: ProposalContent<'a>,
) -> Result<Vec<u8>> {
    let mut slides = Vec::new();

    // Title slide
    slides.push(
        SlideContent::new(title)
            .add_bullet(author)
            .layout(SlideLayout::CenteredTitle)
    );

    // Agenda
    slides.push(
        SlideContent::new("Agenda")
            .add_bullet("Executive Summary")
            .add_bullet("Problem Statement")
            .add_bullet("Proposed Solution")
            .add_bullet("Timeline & Budget")
            .add_bullet("Next Steps")
    );

    // Executive Summary
    let mut summary = SlideContent::new("Executive Summary");
    for point in &content.executive_summary {
        summary = summary.add_bullet(*point);
    }
    slides.push(summary);

    // Problem Statement
    let mut problem = SlideContent::new("Problem Statement");
    for point in &content.problem {
        problem = problem.add_bullet(*point);
    }
    slides.push(problem);

    // Solution
    let mut solution = SlideContent::new("Proposed Solution");
    for point in &content.solution {
        solution = solution.add_bullet(*point);
    }
    slides.push(solution);

    // Timeline
    let mut timeline = SlideContent::new("Timeline");
    for (phase, desc) in &content.timeline {
        timeline = timeline.add_bullet(&format!("{}: {}", phase, desc));
    }
    slides.push(timeline);

    // Budget
    let mut budget = SlideContent::new("Budget");
    for (item, amount) in &content.budget {
        budget = budget.add_bullet(&format!("{}: {}", item, amount));
    }
    slides.push(budget);

    // Next Steps
    let mut next = SlideContent::new("Next Steps");
    for step in &content.next_steps {
        next = next.add_bullet(*step);
    }
    slides.push(next);

    // Q&A
    slides.push(
        SlideContent::new("Questions?")
            .add_bullet("Discussion and Q&A")
            .layout(SlideLayout::CenteredTitle)
    );

    create_pptx_with_content(title, slides).map_err(|e| crate::exc::PptxError::InvalidOperation(e.to_string()))
}

/// Create a training material presentation
///
/// Structure:
/// 1. Title slide
/// 2. Learning Objectives
/// 3. Module slides (one per module)
/// 4. Exercises
/// 5. Summary
/// 6. Q&A
pub fn training_material<'a>(
    title: &str,
    author: &str,
    content: TrainingContent<'a>,
) -> Result<Vec<u8>> {
    let mut slides = Vec::new();

    // Title
    slides.push(
        SlideContent::new(title)
            .add_bullet(author)
            .layout(SlideLayout::CenteredTitle)
    );

    // Objectives
    let mut objectives = SlideContent::new("Learning Objectives");
    for obj in &content.objectives {
        objectives = objectives.add_bullet(*obj);
    }
    slides.push(objectives);

    // Modules
    for (module_name, topics) in &content.modules {
        let mut module = SlideContent::new(*module_name);
        for topic in topics {
            module = module.add_bullet(*topic);
        }
        slides.push(module);
    }

    // Exercises
    let mut exercises = SlideContent::new("Exercises");
    for ex in &content.exercises {
        exercises = exercises.add_bullet(*ex);
    }
    slides.push(exercises);

    // Summary
    let mut summary = SlideContent::new("Summary");
    for point in &content.summary {
        summary = summary.add_bullet(*point);
    }
    slides.push(summary);

    // Q&A
    slides.push(
        SlideContent::new("Questions?")
            .layout(SlideLayout::CenteredTitle)
    );

    create_pptx_with_content(title, slides).map_err(|e| crate::exc::PptxError::InvalidOperation(e.to_string()))
}

/// Create a status report presentation
///
/// Structure:
/// 1. Title slide
/// 2. Executive Summary
/// 3. Completed
/// 4. In Progress
/// 5. Blocked/Risks
/// 6. Next Week
/// 7. Metrics
pub fn status_report<'a>(
    title: &str,
    date: &str,
    content: StatusContent<'a>,
) -> Result<Vec<u8>> {
    let mut slides = Vec::new();

    // Title
    slides.push(
        SlideContent::new(title)
            .add_bullet(date)
            .layout(SlideLayout::CenteredTitle)
    );

    // Summary
    let mut summary = SlideContent::new("Executive Summary");
    for point in &content.summary {
        summary = summary.add_bullet(*point);
    }
    slides.push(summary);

    // Completed
    let mut completed = SlideContent::new("Completed ✓");
    for item in &content.completed {
        completed = completed.add_bullet(*item);
    }
    slides.push(completed);

    // In Progress
    let mut progress = SlideContent::new("In Progress");
    for item in &content.in_progress {
        progress = progress.add_bullet(*item);
    }
    slides.push(progress);

    // Blocked
    if !content.blocked.is_empty() {
        let mut blocked = SlideContent::new("Blocked / Risks");
        for item in &content.blocked {
            blocked = blocked.add_bullet(*item);
        }
        slides.push(blocked);
    }

    // Next Week
    let mut next = SlideContent::new("Next Week");
    for item in &content.next_week {
        next = next.add_bullet(*item);
    }
    slides.push(next);

    // Metrics
    if !content.metrics.is_empty() {
        let mut metrics = SlideContent::new("Key Metrics");
        for (metric, value) in &content.metrics {
            metrics = metrics.add_bullet(&format!("{}: {}", metric, value));
        }
        slides.push(metrics);
    }

    create_pptx_with_content(title, slides).map_err(|e| crate::exc::PptxError::InvalidOperation(e.to_string()))
}

/// Create a technical documentation presentation
///
/// Structure:
/// 1. Title slide
/// 2. Overview
/// 3. Architecture
/// 4. Component slides
/// 5. API Examples
/// 6. Best Practices
pub fn technical_doc<'a>(
    title: &str,
    version: &str,
    content: TechnicalContent<'a>,
) -> Result<Vec<u8>> {
    let mut slides = Vec::new();

    // Title
    slides.push(
        SlideContent::new(title)
            .add_bullet(&format!("Version {}", version))
            .layout(SlideLayout::CenteredTitle)
    );

    // Overview
    let mut overview = SlideContent::new("Overview");
    for point in &content.overview {
        overview = overview.add_bullet(*point);
    }
    slides.push(overview);

    // Architecture
    let mut arch = SlideContent::new("Architecture");
    for point in &content.architecture {
        arch = arch.add_bullet(*point);
    }
    slides.push(arch);

    // Components
    for (name, features) in &content.components {
        let mut comp = SlideContent::new(*name);
        for feature in features {
            comp = comp.add_bullet(*feature);
        }
        slides.push(comp);
    }

    // API Examples
    if !content.api_examples.is_empty() {
        let mut api = SlideContent::new("API Reference");
        for (method, desc) in &content.api_examples {
            api = api.add_bullet(&format!("{} - {}", method, desc));
        }
        slides.push(api);
    }

    // Best Practices
    let mut practices = SlideContent::new("Best Practices");
    for practice in &content.best_practices {
        practices = practices.add_bullet(*practice);
    }
    slides.push(practices);

    create_pptx_with_content(title, slides).map_err(|e| crate::exc::PptxError::InvalidOperation(e.to_string()))
}

/// Create a simple presentation with just title and bullet slides
pub fn simple(title: &str, slides: &[(&str, &[&str])]) -> Result<Vec<u8>> {
    let slide_contents: Vec<SlideContent> = slides.iter().map(|(slide_title, bullets)| {
        let mut slide = SlideContent::new(*slide_title);
        for bullet in *bullets {
            slide = slide.add_bullet(*bullet);
        }
        slide
    }).collect();

    create_pptx_with_content(title, slide_contents).map_err(|e| crate::exc::PptxError::InvalidOperation(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_proposal() {
        let content = ProposalContent::default();
        let result = business_proposal("Test Proposal", "Author", content);
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.len() > 1000);
    }

    #[test]
    fn test_training_material() {
        let content = TrainingContent::default();
        let result = training_material("Test Training", "Trainer", content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_status_report() {
        let content = StatusContent::default();
        let result = status_report("Weekly Status", "2025-01-01", content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_technical_doc() {
        let content = TechnicalContent::default();
        let result = technical_doc("API Documentation", "1.0.0", content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_simple() {
        let slides = [
            ("Introduction", &["Point 1", "Point 2"][..]),
            ("Conclusion", &["Summary"][..]),
        ];
        let result = simple("Simple Presentation", &slides);
        assert!(result.is_ok());
    }

    #[test]
    fn test_custom_proposal_content() {
        let content = ProposalContent {
            executive_summary: vec!["We need more budget", "Market opportunity exists"],
            problem: vec!["Current system is outdated", "Losing customers"],
            solution: vec!["New platform", "Modern technology"],
            timeline: vec![("Q1", "Design"), ("Q2", "Build"), ("Q3", "Launch")],
            budget: vec![("Development", "$100,000"), ("Marketing", "$50,000")],
            next_steps: vec!["Approve budget", "Hire team", "Start development"],
        };
        let result = business_proposal("Q1 Budget Proposal", "Finance", content);
        assert!(result.is_ok());
    }
}

