import React from 'react';
import Container from '@material-ui/core/Container';

interface JobViewProps {
  match: {
    params: {
      id: string
    }
  }
}

function JobView({ match }: JobViewProps) {
  const jobId = match.params.id;
  return (
    <Container>
      JobView: {jobId}
    </Container>
  );
}

export default JobView;
