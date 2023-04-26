mod file_parser;
mod metrics;
use log::info;
use crate::metrics::github::Github;
use crate::metrics::npm::Npm;
use crate::metrics::Metrics;
use std::io::Write;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};
use std::fs::File;

#[allow(dead_code)]
fn calcscore(url: String) -> Result<(), String> {
    let mut net_scores = Vec::new();

    let mut f = File::create("/src/url.txt").expect("Unable to create file");
    f.write_all(url.as_bytes()).expect("Unable to write data to file");
    let file_path = "/src/url.txt";

    let file = std::fs::File::open(file_path).map_err(|e| format!("{}", e))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        info!("exploring {}", line);

        // if type is github or npm
        if let Some(domain) = reqwest::Url::parse(&line)
            .map_err(|_| format!("{} is not a url", line))?
            .domain()
        {
            let project: Box<dyn Metrics>;
            // if github
            if domain == "github.com" {
                project = Box::new(
                    Github::with_url(&line)
                        .ok_or(format!("Error while processing url: {}", &line))?,
                );
            } else if domain == "www.npmjs.com" {
                project = Box::new(
                    Npm::with_url(&line).ok_or(format!("Error while processing url: {}", &line))?,
                );
            } else {
                continue;
            }
            // calculate score
            info!("calculating score");
            let mut net_score = HashMap::new();
            let ramp_up: f64 = project.ramp_up_time();
            let correctness: f64 = project.correctness();
            let bus_factor: f64 = project.bus_factor();
            let responsiveness: f64 = project.responsiveness();
            let compatibility: f64 = project.compatibility();
            let reviewed_code: f64 = project.reviewed_code();
            let pinning_practice = project.pinning_practice();
            let score: f64 = ramp_up * 0.05
                + correctness * 0.1
                + bus_factor * 0.1
                + responsiveness * 0.25
                + compatibility * 0.4
                + reviewed_code * 0.2
                + pinning_practice * 0.1;
            net_score.insert("URL", line);
            net_score.insert("NET_SCORE", score.to_string());
            net_score.insert("RAMP_UP_SCORE", ramp_up.to_string());
            net_score.insert("CORRECTNESS_SCORE", correctness.to_string());
            net_score.insert("BUS_FACTOR_SCORE", bus_factor.to_string());
            net_score.insert("RESPONSIVE_MAINTAINER_SCORE", responsiveness.to_string());
            net_score.insert("REVIEWED_CODE_SCORE", reviewed_code.to_string());
            net_score.insert("LICENSE_SCORE", compatibility.to_string());
            net_score.insert("PINNING_PRACTICE_SCORE", pinning_practice.to_string());
            net_scores.push(net_score);
        } else {
            continue;
        }
    }
    // sort by net scores
    info!("sorting by net scores");
    net_scores.sort_by(|a, b| {
        b["NET_SCORE"]
            .parse::<f64>()
            .unwrap()
            .partial_cmp(&a["NET_SCORE"].parse::<f64>().unwrap())
            .unwrap()
    });

    // stdout the output
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    info!("generating output");
    for dict in net_scores {
        handle
            .write_fmt(format_args!("{{\"URL\":{:?}, ", dict.get("URL").unwrap()))
            .unwrap();
        handle
            .write_fmt(format_args!(
                "\"NET_SCORE\":{:.2}, ",
                dict.get("NET_SCORE").unwrap().parse::<f64>().unwrap()
            ))
            .unwrap();
        handle
            .write_fmt(format_args!(
                "\"RAMP_UP_SCORE\":{:.2}, ",
                dict.get("RAMP_UP_SCORE").unwrap().parse::<f64>().unwrap()
            ))
            .unwrap();
        handle
            .write_fmt(format_args!(
                "\"CORRECTNESS_SCORE\":{:.2}, ",
                dict.get("CORRECTNESS_SCORE")
                    .unwrap()
                    .parse::<f64>()
                    .unwrap()
            ))
            .unwrap();
        handle
            .write_fmt(format_args!(
                "\"BUS_FACTOR_SCORE\":{:.2}, ",
                dict.get("BUS_FACTOR_SCORE")
                    .unwrap()
                    .parse::<f64>()
                    .unwrap()
            ))
            .unwrap();
        handle
            .write_fmt(format_args!(
                "\"REVIEWED_CODE\":{:.2}, ",
                dict.get("REVIEWED_CODE_SCORE")
                    .unwrap()
                    .parse::<f64>()
                    .unwrap()
            ))
            .unwrap();
        handle
            .write_fmt(format_args!(
                "\"RESPONSIVE_MAINTAINER_SCORE\":{:.2}, ",
                dict.get("RESPONSIVE_MAINTAINER_SCORE")
                    .unwrap()
                    .parse::<f64>()
                    .unwrap()
            ))
            .unwrap();
        handle
            .write_fmt(format_args!(
                "\"PINNING_PRACTICE_SCORE\":{:.2}, ",
                dict.get("PINNING_PRACTICE_SCORE")
                    .unwrap()
                    .parse::<f64>()
                    .unwrap()
            ))
            .unwrap();
        handle
            .write_fmt(format_args!(
                "\"LICENSE_SCORE\":{}}}\n",
                dict.get("LICENSE_SCORE").unwrap().parse::<f64>().unwrap()
            ))
            .unwrap();
    }
    Ok(())
}