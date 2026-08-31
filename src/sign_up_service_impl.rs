pub mod sign_up_service_impl {

    //initiate sign up request

    //send OTP to confirm sign up via msisdn or email

    //verify OTP to complete sign up

    //resend OTP to an expired OTP request

    //assign MFA application to account

    //MSISDN validation using E.164 format
    pub fn is_valid_msisdn(input: &str) -> bool {
        let digits = match input.strip_prefix('+') {
            Some(value) => value,
            None => return false,
        };

        let length = digits.len();

        // E.164 allows 1–15 decimal digits after the '+'.
        if !(1..=15).contains(&length) {
            return false;
        }

        // All characters must be ASCII digits, and the first cannot be zero.
        digits.as_bytes().first().is_some_and(|first| {
            (b'1'..=b'9').contains(first)
        }) && digits.bytes().all(|byte| byte.is_ascii_digit())
    }

}