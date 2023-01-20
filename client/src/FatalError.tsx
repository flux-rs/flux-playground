import Alert from "@mui/material/Alert";
import Snackbar, { SnackbarOrigin } from "@mui/material/Snackbar";

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
        {/* <AlertTitle>Error</AlertTitle> */}
        {message}
      </Alert>
    </Snackbar>
  );
}

export default FatalError;
