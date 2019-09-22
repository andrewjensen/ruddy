import React from 'react';
import { makeStyles } from '@material-ui/core/styles';
import Paper from '@material-ui/core/Paper';
import Typography from '@material-ui/core/Typography';

const useStyles = makeStyles(theme => ({
  root: {
    width: '20rem',
    display: 'inline-block',
    marginRight: theme.spacing(3),
    marginBottom: theme.spacing(3),
    padding: theme.spacing(3, 2)
  },
}));

interface CreateJobButtonProps {
  onClick(): void
}

export default function CreateJobButton({ onClick }: CreateJobButtonProps) {
  const classes = useStyles();

  return (
    <Paper className={classes.root} onClick={onClick}>
      <Typography variant="h5">Create new job</Typography>
    </Paper>
  );
}
