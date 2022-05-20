use lettre::SmtpTransport;

use lettre::Message;
use lettre::Transport;

pub(crate) fn handle_error<E: std::error::Error>(error: E) {
    // Email admin if there's an exception
    let email = Message::builder()
        .from(
            "Security Challenges Platform <cs6443@cse.unsw.edu.au>"
                .parse()
                .unwrap(),
        )
        .to("z5420301@ad.unsw.edu.au".parse().unwrap())
        .subject("Exception Occurred in SCP CGI Service")
        .body(format!("{:#?}", error))
        .unwrap();

    let mailer = SmtpTransport::relay("maillard").unwrap().build();

    mailer.send(&email).unwrap();
}
