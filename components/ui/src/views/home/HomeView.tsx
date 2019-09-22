import React from 'react';
import Container from '@material-ui/core/Container';
import Typography from '@material-ui/core/Typography';

import { Job } from '../../common/interfaces';
import JobSummary from './components/JobSummary';
import CreateJobButton from './components/CreateJobButton';

const MOCK_JOBS = require('../../mock-jobs.json');

interface HomeViewProps {
  history: any // TODO: lock down
}

function HomeView({ history }: HomeViewProps) {
  const onSelectJob = (jobId: string) => {
    history.push(`/job/${jobId}`);
  };

  const onCreateJob = () => {
    console.log('onCreateJob');
  };

  return (
    <Container>
      <JobList
        jobs={MOCK_JOBS.jobs}
        onSelectJob={onSelectJob}
        onCreateJob={onCreateJob}
      />
    </Container>
  );
}

export default HomeView;

interface JobListProps {
  jobs: Job[],
  onSelectJob(jobId: string): void,
  onCreateJob(): void
}

function JobList({ jobs, onSelectJob, onCreateJob }: JobListProps) {
  return (
    <div>
      <Typography variant="h5" gutterBottom>Jobs</Typography>
      {jobs.map(job => (
        <JobSummary
          key={job.id}
          job={job}
          onClick={() => onSelectJob(job.id)}
        />
      ))}
      <CreateJobButton
        onClick={onCreateJob}
      />
    </div>
  )
}
