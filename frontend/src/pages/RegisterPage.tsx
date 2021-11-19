import React, { useState } from "react";
import ArchytexAppBar from "../components/ArchytexAppBar";
import ArchytexFooter from "../components/ArchytexFooter";
import RegisterForm from "../components/register-page-components/RegisterForm";

export default function RegisterPage() {
  const [open, setOpen] = useState(false);
  function handleOpenChange(value: boolean): void {
    setOpen(value);
  }
  return (
    <React.Fragment>
      <ArchytexAppBar open={open} handleOpenChange={handleOpenChange} />
      <RegisterForm />
      <ArchytexFooter />
    </React.Fragment>
  );
}
