import Input from "@mui/material/Input";
import IconButton from "@mui/material/IconButton";
import Paper from "@mui/material/Paper";
import ContentCopy from "@mui/icons-material/ContentCopy";
import CloseIcon from "@mui/icons-material/Close";
import Tooltip from "@mui/material/Tooltip";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import { FormControl } from "@mui/material";
import { useRef, useState } from "react";

interface IShareDialogProps {
  link?: string;
  onClose?: () => void;
}

function ShareDialog({ link, onClose }: IShareDialogProps) {
  return (
    <Dialog open={link !== undefined} maxWidth="md" fullWidth onClose={onClose}>
      <DialogTitle>
        Share Link
        <IconButton
          aria-label="close"
          onClick={onClose}
          sx={{
            position: "absolute",
            right: 8,
            top: 8,
            color: (theme) => theme.palette.grey[500],
          }}
        >
          <CloseIcon />
        </IconButton>
      </DialogTitle>
      <Paper sx={{ padding: "0 24px 24px 24px" }}>
        <FormControl variant="filled" fullWidth>
          <Input value={link || ""} endAdornment={<CopyToClipboard text={link} />} />
        </FormControl>
      </Paper>
    </Dialog>
  );
}

function CopyToClipboard({ text }: { text?: string }) {
  const timeoutRef = useRef(null as any);
  const [tooltipOpen, setTooltipOpen] = useState(false);
  const handleClick = () => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }
    setTooltipOpen(true);
    timeoutRef.current = setTimeout(() => setTooltipOpen(false), 1000);
    navigator.clipboard.writeText(text || "");
  };

  return (
    <Tooltip open={tooltipOpen} title="copied">
      <IconButton onClick={handleClick}>
        <ContentCopy />
      </IconButton>
    </Tooltip>
  );
}

export default ShareDialog;
