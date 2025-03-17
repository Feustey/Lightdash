import './globals.css';
import type { Metadata } from 'next';

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
    <html lang="en">
      <body className="font-sans antialiased">{children}</body>
    </html>
  );
}