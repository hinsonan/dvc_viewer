'use client'
import { useState, useEffect } from 'react';
import SimpleAccordion, {DataItem} from '@/components/accordian/accordian';
import BasicButton from '@/components/button/button';
import Navbar from '@/components/navbar/navbar';
import BasicTextFields from '@/components/text_input/basic_text_input';

export default function Home() {
  const [data, setDvcData] = useState<DataItem[]>([]);
  const [textFieldValue, setTextFieldValue] = useState('');

  const handleClick = async () => {

    try {
      console.log(JSON.stringify(textFieldValue.trimEnd()))
      const response = await fetch('http://127.0.0.1:8000/clone', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: textFieldValue.trim(),
      });

      if (response.ok) {
        // Handle successful response
        setDvcData(await response.json())
        console.log('Post created successfully');
        console.log(data)
      } else {
        // Handle error response
        console.log('Failed to create post');
      }
    } catch (error) {
      // Handle fetch error
      console.error('Error:', error);
    }
  };
  const handleUrlChange = (newUrl: string) => {
    // Update the url state in the parent component
    setTextFieldValue(newUrl);
  };

  return (
    <div>
      <Navbar />
      <main className="flex flex-col items-center justify-between p-24">
        <div>
          <BasicTextFields onUrlChange={handleUrlChange}/>
          <div className="flex flex-col float-right">
            <BasicButton onClick={handleClick}/>
          </div>
        </div>
        {data.length !== 0 && <SimpleAccordion data={data}/>}
      </main>
    </div>
  )
}
