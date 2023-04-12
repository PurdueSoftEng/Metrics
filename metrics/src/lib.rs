mod metrics {
    pub mod github;
    pub mod npm;
}

use metrics::{github, npm};
use crate::metrics::github::Github;

fn calc_ramp_up_time() {
    let github = Github {
        owner: "my_username".to_string(),
        repo: "my_repo".to_string(),
        link: "https://github.com/my_username/my_repo.git".to_string(),
    };
    let ramp_up_time = github.ramp_up_time();
    println!("Ramp up time: {}", ramp_up_time);
}

mod japanese {
    mod greetings {
    }

    mod farewells {
    }
}