"use client";

import React, { useState, useEffect } from "react";
import Link from "next/link";
import { usePathname, useRouter } from "next/navigation";
import { 
  Zap, 
  Activity, 
  MessageCircle, 
  Settings, 
  Menu, 
  Lightbulb, 
  Search,
  User,
  Bell,
  ChevronDown,
  Moon,
  Sun,
  Loader2,
  MenuSquare
} from "lucide-react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { 
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from "@/components/ui/command";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import { fetchNodeData, fetchDefaultNodeData } from "@/app/utils/sparkseer";

interface LayoutProps {
  children: React.ReactNode;
}

interface SearchResult {
  pubkey: string;
  alias: string;
  capacity: number;
}

export function Layout({ children }: LayoutProps) {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);
  const [searchQuery, setSearchQuery] = useState("");
  const [theme, setTheme] = useState<"light" | "dark">("light");
  const [searchResults, setSearchResults] = useState<SearchResult[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  const [open, setOpen] = useState(false);
  const [defaultNodeData, setDefaultNodeData] = useState<any>(null);
  const pathname = usePathname();
  const router = useRouter();

  useEffect(() => {
    const loadDefaultNode = async () => {
      const data = await fetchDefaultNodeData();
      if (data) {
        setDefaultNodeData(data);
        setSearchQuery(data.alias || data.publicKey);
      }
    };
    loadDefaultNode();
  }, []);

  const handleSearchChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const query = e.target.value;
    setSearchQuery(query);
    setOpen(true);

    if (query.length < 2) {
      setSearchResults([]);
      return;
    }

    setIsSearching(true);
    try {
      const response = await fetch(`https://1ml.com/search?q=${encodeURIComponent(query)}`);
      const data = await response.json();
      setSearchResults(data.results || []);
    } catch (error) {
      console.error('Erreur lors de la recherche:', error);
    } finally {
      setIsSearching(false);
    }
  };

  const handleNodeSelect = async (node: SearchResult) => {
    setSearchQuery(node.alias || node.pubkey);
    setOpen(false);
    setSearchResults([]);

    try {
      const nodeData = await fetchNodeData(node.pubkey);
      if (nodeData) {
        router.push(`/channels/${node.pubkey}`);
      }
    } catch (error) {
      console.error('Erreur lors de la récupération des données du nœud:', error);
    }
  };

  const toggleTheme = () => {
    setTheme(theme === "light" ? "dark" : "light");
    // Ici, vous pourriez implémenter la logique pour changer le thème de l'application
  };

  return (
    <div className="flex min-h-screen bg-gray-50 dark:bg-gray-900" suppressHydrationWarning>
      {/* Sidebar */}
      <div
        className={cn(
          "fixed left-0 top-0 z-40 h-screen w-64 transform bg-white dark:bg-gray-800 shadow-lg transition-transform duration-200 ease-in-out",
          isSidebarOpen ? "translate-x-0" : "-translate-x-full"
        )}
      >
        <div className="flex h-16 items-center border-b px-6">
          <Zap className="h-6 w-6 text-blue-600" />
          <span className="ml-2 text-lg font-bold text-gray-900 dark:text-white">Lightning Manager</span>
        </div>
        
        <div className="p-4">
          <div className="relative mb-6">
            <Input
              type="text"
              placeholder="Rechercher..."
              className="pl-10 bg-gray-100 dark:bg-gray-700 border-0"
            />
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
          </div>
        </div>
        
        <nav className="space-y-1 p-4">
          <p className="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-2 pl-3">
            Principal
          </p>
          <Link href="/" passHref>
            <Button
              variant={pathname === "/" ? "default" : "ghost"}
              className="w-full justify-start"
            >
              <Activity className="mr-2 h-4 w-4" />
              Tableau de bord
            </Button>
          </Link>
          <Link href="/channels" passHref>
            <Button
              variant={pathname === "/channels" ? "default" : "ghost"}
              className="w-full justify-start"
            >
              <Zap className="mr-2 h-4 w-4" />
              Canaux
            </Button>
          </Link>
          <Link href="/actions" passHref>
            <Button
              variant={pathname === "/actions" ? "default" : "ghost"}
              className="w-full justify-start"
            >
              <Lightbulb className="mr-2 h-4 w-4" />
              Actions
            </Button>
          </Link>
          
          <p className="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider mt-6 mb-2 pl-3">
            Communication
          </p>
          <Link href="/messages" passHref>
            <Button
              variant={pathname === "/messages" ? "default" : "ghost"}
              className="w-full justify-start"
            >
              <MessageCircle className="mr-2 h-4 w-4" />
              Messages
            </Button>
          </Link>
          
          <p className="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider mt-6 mb-2 pl-3">
            Préférences
          </p>
          <Link href="/settings" passHref>
            <Button
              variant={pathname === "/settings" ? "default" : "ghost"}
              className="w-full justify-start"
            >
              <Settings className="mr-2 h-4 w-4" />
              Paramètres
            </Button>
          </Link>
        </nav>
        
        <div className="absolute bottom-0 left-0 right-0 p-4 border-t">
          <div className="flex items-center">
            <Avatar className="h-8 w-8">
              <AvatarImage src="/avatar.png" alt="Avatar" />
              <AvatarFallback>LN</AvatarFallback>
            </Avatar>
            <div className="ml-3">
              <p className="text-sm font-medium text-gray-900 dark:text-white">
                {defaultNodeData?.alias || "Chargement..."}
              </p>
              <p className="text-xs text-gray-500 dark:text-gray-400">
                {defaultNodeData?.publicKey ? `${defaultNodeData.publicKey.slice(0, 8)}...` : "Nœud actif"}
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Main content */}
      <div className={cn("flex-1", isSidebarOpen ? "ml-64" : "ml-0")}>
        <header className="sticky top-0 z-30 flex h-16 items-center justify-between border-b bg-white dark:bg-gray-800 px-6">
          <div className="flex items-center">
            <Button
              variant="ghost"
              size="icon"
              onClick={() => setIsSidebarOpen(!isSidebarOpen)}
              className="mr-4"
            >
              <Menu className="h-5 w-5" />
            </Button>
            
            <h1 className="text-xl font-semibold text-gray-900 dark:text-white hidden md:block">
              {pathname === "/" && "Tableau de bord"}
              {pathname === "/channels" && "Gestion des canaux"}
              {pathname === "/actions" && "Actions recommandées"}
              {pathname === "/messages" && "Messages"}
              {pathname === "/settings" && "Paramètres"}
            </h1>
          </div>
          
          <div className="flex items-center space-x-4">
            <Popover open={open} onOpenChange={setOpen}>
              <PopoverTrigger asChild>
                <Button
                  variant="outline"
                  role="combobox"
                  aria-expanded={open}
                  className="w-[300px] justify-between"
                >
                  {searchQuery || "Rechercher un nœud..."}
                  <Search className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                </Button>
              </PopoverTrigger>
              <PopoverContent className="w-[300px] p-0">
                <Command>
                  <CommandInput 
                    placeholder="Entrez une clé publique ou un alias..." 
                    value={searchQuery}
                    onValueChange={setSearchQuery}
                  />
                  <CommandList>
                    <CommandEmpty>
                      {isSearching ? (
                        <div className="flex items-center justify-center py-6">
                          <Loader2 className="h-4 w-4 animate-spin" />
                        </div>
                      ) : (
                        "Aucun nœud trouvé."
                      )}
                    </CommandEmpty>
                    <CommandGroup heading="Résultats">
                      {searchResults.map((node) => (
                        <CommandItem
                          key={node.pubkey}
                          value={node.pubkey}
                          onSelect={() => handleNodeSelect(node)}
                        >
                          <div className="flex flex-col">
                            <span className="font-medium">{node.alias || "Nœud anonyme"}</span>
                            <span className="text-sm text-gray-500">{node.pubkey}</span>
                          </div>
                        </CommandItem>
                      ))}
                    </CommandGroup>
                  </CommandList>
                </Command>
              </PopoverContent>
            </Popover>
            
            <Button variant="ghost" size="icon" onClick={toggleTheme}>
              {theme === "light" ? (
                <Moon className="h-5 w-5" />
              ) : (
                <Sun className="h-5 w-5" />
              )}
            </Button>
            
            <Button variant="ghost" size="icon" className="relative">
              <Bell className="h-5 w-5" />
              <span className="absolute top-1 right-1 h-2 w-2 rounded-full bg-red-500"></span>
            </Button>
            
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="ghost" className="flex items-center">
                  <User className="mr-2 h-4 w-4" />
                  <span className="hidden md:inline">Lightning User</span>
                  <ChevronDown className="ml-2 h-4 w-4" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuLabel>Mon compte</DropdownMenuLabel>
                <DropdownMenuSeparator />
                <DropdownMenuItem>
                  <User className="mr-2 h-4 w-4" />
                  <span>Profil</span>
                </DropdownMenuItem>
                <DropdownMenuItem>
                  <Settings className="mr-2 h-4 w-4" />
                  <span>Paramètres</span>
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem>
                  <span>Déconnexion</span>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </header>
        
        <main className="p-6">
          {children}
        </main>
      </div>
    </div>
  );
}