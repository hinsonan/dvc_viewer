import * as React from 'react';
import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField';

export default function BasicTextFields() {
  const [url, setUrl] = React.useState('');
  const [isValidUrl, setIsValidUrl] = React.useState(true);

  function checkUrlValidity(url: string): boolean {
    // Regular expression pattern for URL validation
    const urlPattern = /^(ftp|http|https):\/\/[^ "]+$/;

    return urlPattern.test(url);
  }

  const handleUrlChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const inputValue = event.target.value;
    setUrl(inputValue);

    // Check if the input value is a valid URL
    setIsValidUrl(checkUrlValidity(inputValue));
  };

  return (
    <Box
      component="form"
      sx={{
        '& > :not(style)': { m: 1, width: '25ch' },
        '& .MuiInputLabel-root': { color: 'white' },
      }}
      noValidate
      autoComplete="off"
    >
      <TextField
        sx={{
          '& .MuiOutlinedInput-root': {
            color: 'white',
            '& fieldset': {
              borderColor: 'white',
            },
            '&:hover fieldset': {
              borderColor: 'white',
            },
          },
        }}
        value={url}
        onChange={handleUrlChange}
        error={!isValidUrl}
        helperText={!isValidUrl && 'Invalid URL'}
        id="outlined-basic"
        label="DVC Git Repository"
        variant="outlined"
      />
    </Box>
  );
}
