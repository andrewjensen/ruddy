import React from 'react';
import {
  BrowserRouter as Router,
  Route
} from 'react-router-dom';
import { makeStyles } from '@material-ui/core';

import TopNav from './common/components/TopNav';
import HomeView from './views/home/HomeView';
import JobView from './views/job/JobView';

function App() {
  return (
    <div className="App">
      <TopNav />

      <View>
        <AppRouter />
      </View>
    </div>
  );
}

export default App;

function AppRouter() {
  return (
    <Router>
      <Route path="/" exact component={HomeView} />
      <Route path="/job/:id" component={JobView} />
    </Router>
  );
}

interface ViewProps {
  children: JSX.Element
}

const viewStyles = makeStyles(theme => ({
  view: {
    paddingTop: theme.spacing(4)
  }
}));

function View({ children }: ViewProps) {
  const classes = viewStyles();
  return (
    <div className={classes.view}>
      {children}
    </div>
  )
}
