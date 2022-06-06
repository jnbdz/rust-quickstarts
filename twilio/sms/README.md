# Simple | SMS | Twilio | Rust | Quickstarts
> NOTE: Twilio OpenAPI Spec is at the present in **BETA**. It is unsupported and under active development. For more details follow the link in the Resources section at the bottom of this README.

> With https://github.com/twilio/twilio-oai it is now possible to auto-generate your own Twilio client in your preferred programming language.

> For now Rust is not supported meaning it doesn't not have an official SDK. But with [OpenAPI Generator](https://openapi-generator.tech/) you can auto-generate one for Rust.

## `.env`
You need this file to be apple to run the Twilio application.

At the root of your Rust project: 
```bash
vim .env
```
Inside you will add: 
```bash
TWILIO_API_KEY=SKXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
TWILIO_API_KEY_SECRET=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
TWILIO_ACCOUNT_SID=ACXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
TWILIO_PHONE_NUMBER=+1098765432
TO_NUMBER=+1234567890
```
> **NOTE:** The phone numbers are just examples. You need to have them in your Twilio account.

## Resources
- [Generating a Rust client for Twilio's API | Twilio](https://www.twilio.com/docs/openapi/generating-a-rust-client-for-twilios-api)
