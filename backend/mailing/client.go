package mailing

import (
	"crypto/tls"
	"fmt"
	"html/template"
	"io"
	"net"
	"net/smtp"
	"os"
)

const (
	MIME = "MIME-version: 1.0;\nContent-Type: text/html; charset=\"UTF-8\";\n\n"
)

func GetClient(rcpt, subject string) (*smtp.Client, io.WriteCloser, error) {
	server := os.Getenv("SMTP_SERVER")
	host, _, err := net.SplitHostPort(server)
	if err != nil {
		return nil, nil, err
	}
	address := os.Getenv("SMTP_ADDRESS")
	password := os.Getenv("SMTP_PASSWORD")
	c, err := smtp.Dial(server)
	if err != nil {
		return nil, nil, err
	}
	tlsconfig := tls.Config{
		InsecureSkipVerify: true,
		ServerName:         host,
	}
	c.StartTLS(&tlsconfig)
	auth := smtp.PlainAuth("", address, password, host)
	err = c.Auth(auth)
	if err != nil {
		c.Close()
		return nil, nil, err
	}
	err = c.Mail(address)
	if err != nil {
		c.Close()
		return nil, nil, err
	}
	err = c.Rcpt(rcpt)
	if err != nil {
		c.Close()
		return nil, nil, err
	}
	wc, err := c.Data()
	if err != nil {
		c.Close()
		return nil, nil, err
	}
	header := fmt.Sprintf("From: %s\nTo: %s\nSubject: %s\n%s", address, rcpt, subject, MIME)
	_, err = wc.Write([]byte(header))
	if err != nil {
		wc.Close()
		c.Close()
		return nil, nil, err
	}
	return c, wc, nil
}

func SendTemplate(rcpt, subject, templateName string, data interface{}) error {
	tmpl, err := template.ParseFiles("templates/emails.tmpl")
	if err != nil {
		return err
	}
	c, wc, err := GetClient(rcpt, subject)
	if err != nil {
		return err
	}
	defer c.Close()
	defer wc.Close()
	err = tmpl.ExecuteTemplate(wc, templateName, data)
	if err != nil {
		return err
	}
	return nil
}
