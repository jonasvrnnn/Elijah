use maud::{Markup, html};

use crate::forms::service::FormSubmission;

pub fn submissions_overview(submissions: &Vec<FormSubmission>) -> Markup {
    html!(
        @for submission in submissions {
            tr hx-get={"/api/forms/submissions/"(submission.id)"/message"}{
                td { (submission.datetime.format("%Y/%m/%d")) }
                td { (submission.first_name) " " (submission.last_name)}
                td {
                    @if let Some(sent_to) = &submission.sent_to {
                        "("(submission.company)") "(sent_to)
                    }
                }
                td { @if let Some(recaptcha_score) = submission.recaptcha_score {
                        (recaptcha_score)"%"
                    }
                }
            }
        }
    )
}
