import React, { useState } from "react";
import ArchytexAppBar from "../components/ArchytexAppBar";
import ArchytexFooter from "../components/ArchytexFooter";
import LoginForm from "../components/login-page-components/LoginForm";

export default function LoginPage() {
  const [open, setOpen] = useState(false);
  function handleOpenChange(value: boolean): void {
    setOpen(value);
  }

  return (
    <React.Fragment>
      <ArchytexAppBar content="general" />
      <LoginForm />
      <ArchytexFooter />
    </React.Fragment>
  );
}
