"use client";

import { useState } from "react";
import { Bolt, Activity, MessageCircle, Settings, Menu } from "lucide-react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";

interface LayoutProps {
  children: React.ReactNode;
}

export function Layout({ children }: LayoutProps) {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);

  return (
    <div className="flex min-h-screen">
      {/* Sidebar */}
      <div
        className={cn(
          "fixed left-0 top-0 z-40 h-screen w-64 transform bg-background transition-transform duration-200 ease-in-out",
          isSidebarOpen ? "translate-x-0" : "-translate-x-full"
        )}
      >
        <div className="flex h-16 items-center border-b px-6">
          <Bolt className="h-6 w-6" />
          <span className="ml-2 text-lg font-bold">Lightning Manager</span>
        </div>
        <nav className="space-y-1 p-4">
          <Button
            variant="ghost"
            className="w-full justify-start"
            onClick={() => {}}
          >
            <Activity className="mr-2 h-4 w-4" />
            Overview
          </Button>
          <Button
            variant="ghost"
            className="w-full justify-start"
            onClick={() => {}}
          >
            <Bolt className="mr-2 h-4 w-4" />
            Channels
          </Button>
          <Button
            variant="ghost"
            className="w-full justify-start"
            onClick={() => {}}
          >
            <MessageCircle className="mr-2 h-4 w-4" />
            Messages
          </Button>
          <Button
            variant="ghost"
            className="w-full justify-start"
            onClick={() => {}}
          >
            <Settings className="mr-2 h-4 w-4" />
            Settings
          </Button>
        </nav>
      </div>

      {/* Main content */}
      <div className={cn("flex-1", isSidebarOpen ? "ml-64" : "ml-0")}>
        <header className="flex h-16 items-center border-b px-6">
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setIsSidebarOpen(!isSidebarOpen)}
          >
            <Menu className="h-6 w-6" />
          </Button>
        </header>
        <main>{children}</main>
      </div>
    </div>
  );
}