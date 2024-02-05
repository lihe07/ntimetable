use std::{io::Write, path::Path};

use serde::Serialize;

use crate::{
    optimize::{Solution, TIMEMAP},
    project::Project,
};

static mut LOG: Option<Log> = None;

pub fn now_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[derive(Debug, Serialize)]
pub struct Step {
    pub i: usize,
    pub weights: Vec<f32>,
    pub neighborhood_average: Vec<f32>,
    pub history_size: usize,
    pub neighborhood_grading_time: u128,
    pub mosa_time: u128,
    pub average_scores: Vec<f32>,
    pub max_scores: Vec<f32>,
    pub graded: usize,
    pub temperature: f32,
}

#[derive(Default, Debug, Serialize)]
struct Log {
    initial_method: String,
    initial_time: u128,
    initial_scores: Vec<f32>,

    steps: Vec<Step>,
    solutions: Vec<TIMEMAP>,
    solutions_scores: Vec<Vec<f32>>,
}

pub fn initial(project: &Project, time: u128, s: TIMEMAP) {
    unsafe {
        if LOG.is_none() {
            LOG = Some(Log::default());
        }
        let log = LOG.as_mut().unwrap();
        log.initial_method = project.config.initial_method.to_owned();
        log.initial_time = time;
        log.initial_scores = project.criteria().evaluate(&Solution::new(s), project);
    }
}

pub fn step(step: Step) {
    unsafe {
        let log = LOG.as_mut().unwrap();
        log.steps.push(step);
    }
}

pub fn finish<P: AsRef<Path>>(project: &Project, path: P, solutions: Vec<TIMEMAP>) {
    unsafe {
        let log = LOG.as_mut().unwrap();
        log.solutions = solutions.clone();
        log.solutions_scores = solutions
            .iter()
            .map(|s| {
                project
                    .criteria()
                    .evaluate(&Solution::new(s.clone()), project)
            })
            .collect();

        let json = serde_json::to_string(log).unwrap();

        let mut file = std::fs::File::create(path.as_ref().join("log.json")).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
