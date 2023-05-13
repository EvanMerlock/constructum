#!/bin/sh

openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256 -days 3650 -nodes -nodes -subj "/C=US/ST=MN/L=Minneapolis/O=ElectronLabs/OU=SelfSigned/CN=localhost"