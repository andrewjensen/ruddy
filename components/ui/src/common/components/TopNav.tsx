import React from 'react';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';

function TopNav() {
  return (
    <div className="Topbar">
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6">
            Ruddy
          </Typography>
        </Toolbar>
      </AppBar>
    </div>
  )
}

export default TopNav;
