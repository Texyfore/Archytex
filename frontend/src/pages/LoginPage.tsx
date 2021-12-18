import React from "react";
import ArchytexAppBar from "../components/ArchytexAppBar";
import ArchytexFooter from "../components/ArchytexFooter";
import LoginForm from "../components/login-page-components/LoginForm";

export default function LoginPage() {
  return (
    <React.Fragment>
      <ArchytexAppBar content="general" />
      <LoginForm />
      <ArchytexFooter />
    </React.Fragment>
  );
}
