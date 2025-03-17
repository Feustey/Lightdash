/// <reference types="react" />
/// <reference types="react-dom" />
/// <reference types="next" />

declare module 'recharts' {
  export * from 'recharts';
}

declare module 'lucide-react' {
  export * from 'lucide-react';
}

declare module 'react' {
  export * from 'react';
  export const useState: <T>(initialState: T | (() => T)) => [T, React.Dispatch<React.SetStateAction<T>>];
  export const useEffect: typeof import("react").useEffect;
  export const useRef: typeof import("react").useRef;
  export const useCallback: typeof import("react").useCallback;
  export const useMemo: typeof import("react").useMemo;
  export const useContext: typeof import("react").useContext;
  export const useReducer: typeof import("react").useReducer;
  export const useLayoutEffect: typeof import("react").useLayoutEffect;
  export const useImperativeHandle: typeof import("react").useImperativeHandle;
  export const useDebugValue: typeof import("react").useDebugValue;
  export const useId: typeof import("react").useId;
  export const useSyncExternalStore: typeof import("react").useSyncExternalStore;
  export const useInsertionEffect: typeof import("react").useInsertionEffect;
  export const useDeferredValue: typeof import("react").useDeferredValue;
  export const useTransition: typeof import("react").useTransition;
  export const use: typeof import("react").use;
}

declare module 'react-dom' {
  export * from 'react-dom';
}

declare module 'next' {
  export * from 'next';
}

declare module '@/*' {
  export * from '@/*';
}

declare module "@/components/ui/badge" {
  export interface BadgeProps extends React.HTMLAttributes<HTMLDivElement> {
    variant?: 'default' | 'secondary' | 'destructive' | 'outline';
  }
  export const Badge: React.FC<BadgeProps>;
}

declare module "lucide-react" {
  export const Activity: React.FC<React.SVGProps<SVGSVGElement>>;
  export const Bolt: React.FC<React.SVGProps<SVGSVGElement>>;
  export const MessageCircle: React.FC<React.SVGProps<SVGSVGElement>>;
  export const Settings: React.FC<React.SVGProps<SVGSVGElement>>;
  export const Search: React.FC<React.SVGProps<SVGSVGElement>>;
  export const Check: React.FC<React.SVGProps<SVGSVGElement>>;
  export const Lightbulb: React.FC<React.SVGProps<SVGSVGElement>>;
  export const Globe: React.FC<React.SVGProps<SVGSVGElement>>;
  export const Moon: React.FC<React.SVGProps<SVGSVGElement>>;
  export const Sun: React.FC<React.SVGProps<SVGSVGElement>>;
  export const RefreshCw: React.FC<React.SVGProps<SVGSVGElement>>;
  export const ArrowRight: React.FC<React.SVGProps<SVGSVGElement>>;
  export const Zap: React.FC<React.SVGProps<SVGSVGElement>>;
  export const Loader2: React.FC<React.SVGProps<SVGSVGElement>>;
  export const AlertTriangle: React.FC<React.SVGProps<SVGSVGElement>>;
  export const ArrowUpRight: React.FC<React.SVGProps<SVGSVGElement>>;
  export const ArrowDownRight: React.FC<React.SVGProps<SVGSVGElement>>;
  export const CheckCircle: React.FC<React.SVGProps<SVGSVGElement>>;
  export const AlertCircle: React.FC<React.SVGProps<SVGSVGElement>>;
  export const TestTube: React.FC<React.SVGProps<SVGSVGElement>>;
}

declare module "@/components/theme-provider" {
  export const ThemeProvider: React.FC<{
    children: React.ReactNode;
    attribute?: string;
    defaultTheme?: string;
    enableSystem?: boolean;
    disableTransitionOnChange?: boolean;
  }>;
}

declare module "@/components/LanguageSelector" {
  export const LanguageSelector: React.FC<{
    currentLanguage: string;
    onLanguageChange: (language: string) => void;
  }>;
}

declare module "@/components/ThemeToggle" {
  export const ThemeToggle: React.FC;
}

declare module "@/components/LightningAuth" {
  export interface LightningAuthProps {
    onAuth: (success: boolean, pubkey: string) => void;
    onPremiumAccess: (success: boolean) => void;
  }
  export const LightningAuth: React.FC<LightningAuthProps>;
} 