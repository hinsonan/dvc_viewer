import * as React from 'react';
import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField';

export default function BasicTextFields() {
  return (
    <Box
      component="form"
      sx={{
        '& > :not(style)': { m: 1, width: '25ch' },
        '& .MuiInputLabel-root': {color: 'white'}
      }}
      noValidate
      autoComplete="off"
    >
      <TextField 
      sx={{
          '& .MuiOutlinedInput-root': {
            color:'white',
            '& fieldset': {
              borderColor: 'white',
            },
            '&:hover fieldset': {
              borderColor: 'white',
            },
          },
      }}
      id="outlined-basic" label="DVC Git Repository" variant="outlined"/>
    </Box>
  );
}