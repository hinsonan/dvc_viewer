import * as React from 'react';
import { styled } from '@mui/material/styles';
import Button from '@mui/material/Button';
import Stack from '@mui/material/Stack';

const CustomButton = styled(Button)({
    boxShadow: 'none',
    textTransform: 'none',
    fontSize: 16,
    padding: '6px 12px',
    border: '2px solid',
    lineHeight: 1.5,
    backgroundColor: '#0063cc !important',
    borderColor: '#0063cc',
    '&:hover': {
      backgroundColor: '#0D47A1 !important',
      borderColor: '#0D47A1 !important',
      boxShadow: 'none',
    },
    '&:active': {
      boxShadow: 'none',
      backgroundColor: '#0062cc',
      borderColor: '#005cbf',
    },
    '&:focus': {
      boxShadow: '0 0 0 0.2rem rgba(0,123,255,.5)',
    },
  });

type CustomButtonProps = {
  onClick: () => void; // Define the type for the custom function prop
};

export default function BasicButton({ onClick }: CustomButtonProps) {
  return (
    <Stack spacing={2} direction="row">
      <CustomButton variant="contained" onClick={onClick}>Scan DVC Repo</CustomButton>
    </Stack>
  );
}