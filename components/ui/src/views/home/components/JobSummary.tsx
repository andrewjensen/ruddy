import React from 'react';
import { makeStyles } from '@material-ui/core/styles';
import Paper from '@material-ui/core/Paper';

import { Job } from '../../../common/interfaces';

const jobSummaryStyles = makeStyles(theme => ({
  root: {
    width: '20rem',
    display: 'inline-block',
    marginRight: theme.spacing(3),
    marginBottom: theme.spacing(3),
    padding: theme.spacing(3, 2),
    "&:hover": {
      backgroundColor: '#f0f0f0',
      cursor: 'pointer'
    }
  }
}));

const progressBarStyles = makeStyles(theme => ({
  container: {
    height: 10,
    backgroundColor: 'grey',
    borderRadius: 6
  },
  complete: {
    height: 10,
    backgroundColor: 'blue',
    borderTopLeftRadius: 6,
    borderBottomLeftRadius: 6
  }
}));

interface JobSummaryProps {
  job: Job,
  onClick(): void
}

export default function JobSummary({ job, onClick }: JobSummaryProps) {
  const classes = jobSummaryStyles();

  return (
    <Paper className={classes.root} onClick={onClick}>
      <div>Job {job.id}</div>
      <div>
        <img
          src={job.thumbnail}
          style={{ width: '100%' }}
          alt="Job thumbnail"
        />
      </div>
      <JobSummaryStatus
        job={job}
      />
    </Paper>
  );
}

interface JobSummaryStatusProps {
  job: Job
}

function JobSummaryStatus({ job }: JobSummaryStatusProps) {
  const { status } = job;
  if (status === 'RENDERING') {
    const renderStatus = job.render;
    const completed = renderStatus.framesRendered;
    const total = (renderStatus.frameEnd - renderStatus.frameStart + 1);
    return (
      <div>
        <div>Status: Rendering ({getPercentText(completed, total)})</div>
        <ProgressBar
          completed={completed}
          total={total}
        />
      </div>
    );
  } else {
    throw new Error('Cannot get status text');
  }
}

function getPercentText(completed: number, total: number) {
  const percent = Math.floor(completed / total * 100.0);
  return `${percent}%`;
}

interface ProgressBarProps {
  completed: number,
  total: number
}

function ProgressBar({ completed, total }: ProgressBarProps) {
  const classes = progressBarStyles();

  return (
    <div className={classes.container}>
      <div
        className={classes.complete}
        style={{ width: getPercentText(completed, total) }}
      ></div>
    </div>
  );
}
