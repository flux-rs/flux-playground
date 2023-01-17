import { Alert, Snackbar, SnackbarOrigin, AlertTitle, SnackbarCloseReason } from "@mui/material";

interface FatalErrorPros {
  message?: string;
  onClose?: (event: React.SyntheticEvent<any> | Event, reason?: string) => void;
}

function FatalError({ message, onClose }: FatalErrorPros) {
  const anchor: SnackbarOrigin = { vertical: "bottom", horizontal: "left" };

  return (
    <Snackbar
      open={message !== undefined}
      anchorOrigin={anchor}
      autoHideDuration={6000}
      onClose={onClose}
    >
      <Alert severity="error" onClose={onClose} sx={{ width: "100%" }} variant="filled">
        <AlertTitle>Internal Server Error</AlertTitle>
        {message}
      </Alert>
    </Snackbar>
  );
}

export default FatalError;
