import './globals.css';
import { Metadata } from 'next/types';

export const metadata: Metadata = {
  title: 'Lightning Node Manager',
  description: 'Monitor and manage your Lightning Network node',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="h-full">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta httpEquiv="X-UA-Compatible" content="IE=edge" />
      </head>
      <body className="font-sans antialiased h-full">
        <div className="min-h-full">
          {children}
        </div>
      </body>
    </html>
  );
}