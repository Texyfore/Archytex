import React from "react";
import ArchytexAppBar from "../components/ArchytexAppBar";
import ArchytexFooter from "../components/ArchytexFooter";
import RegisterForm from "../components/register-page-components/RegisterForm";

export default function RegisterPage() {
  return (
    <React.Fragment>
      <ArchytexAppBar content="general" />
      <RegisterForm />
      <ArchytexFooter />
    </React.Fragment>
  );
}
