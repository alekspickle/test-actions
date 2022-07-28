use octocrab::{
    models::{
        workflows::{Job, Step},
        JobId,
    },
    Octocrab,
};

/// Struct aggregating useful job info
#[derive(Debug)]
pub struct JobInfo {
    pub id: JobId,
    pub run_url: String,
    pub steps: Vec<StepInfo>,
    pub conclusion: Option<String>,
}

impl From<Job> for JobInfo {
    fn from(job: Job) -> Self {
        Self {
            id: job.id,
            run_url: job.run_url.to_string(),
            steps: job.steps.into_iter().map(Into::into).collect(),
            conclusion: job.conclusion,
        }
    }
}

#[derive(Debug)]
pub struct StepInfo {
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
}

impl From<Step> for StepInfo {
    fn from(s: Step) -> Self {
        Self {
            name: s.name,
            status: s.status,
            conclusion: s.conclusion,
        }
    }
}
