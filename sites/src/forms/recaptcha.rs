use std::fmt::Display;

use serde::Deserialize;

pub enum RecaptchaError {
    ErrorCodes(Option<Vec<String>>),
    ScoreTooLow(f32),
    NoScoreProperty,
    DeserialisationError(reqwest::Error),
    PostError(reqwest::Error),
}

impl Display for RecaptchaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecaptchaError::ErrorCodes(items) => {
                let error_codes = items.clone()
                    .map(|ec| format!("\n{}", ec.join("\n")))
                    .unwrap_or("[NO ERROR CODES FOUND]".to_string());

                write!(f, "The request to Google's ReCaptcha server failed with the following error codes: {error_codes}.")
            },
            RecaptchaError::ScoreTooLow(score) => {
                write!(f, "The score returned by Google's ReCaptcha service was lower than the given minimum score: was {score}, but should have been at least 0.7.")
            },
            RecaptchaError::NoScoreProperty => {
                write!(f, "The response returned by Google's ReCaptcha service was correctly deserialised, but did not contain a score property.")
            },
            RecaptchaError::DeserialisationError(err) => {
                write!(f, "Failed to deserialise the response for the ReCaptcha verification: {err}.")  
            },
            RecaptchaError::PostError(err) => {
                write!(f, "Failed to make the POST request to Google's ReCaptcha server: {err}.")
            },
        }
    }
}

#[derive(Deserialize, Debug)]
struct ReCaptchaResponse {
    success: bool,
    error_codes: Option<Vec<String>>,
    score: Option<f32>,
}

pub async fn verify_token(secret: &str, token: &str) -> Result<f32, RecaptchaError> {
    let client = reqwest::Client::new();

    let return_score;

    match client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .query(&[("secret", secret), ("response", token)])
        .header("Content-Length", 0)
        .send()
        .await
    {
        Ok(res) => match res.json::<ReCaptchaResponse>().await {
            Ok(response) => {
                if !response.success {
                    return Err(RecaptchaError::ErrorCodes(response.error_codes));
                }

                if let Some(score) = response.score {
                    return_score = score;
                } else {
                    return Err(RecaptchaError::NoScoreProperty);
                }
            }
            Err(err) => {
                return Err(RecaptchaError::DeserialisationError(err));
            }
        },
        Err(err) => {
            return Err(RecaptchaError::PostError(err));
        }
    };

    Ok(return_score)
}
