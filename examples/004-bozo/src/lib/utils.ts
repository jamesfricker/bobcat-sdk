import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatAddress(address: string): string {
  return `${address.slice(0, 6)}...${address.slice(-4)}`;
}

export function formatTokenAmount(amount: string, decimals: number = 4): string {
  const num = parseFloat(amount);
  return num.toFixed(decimals);
}

export function formatTokenValue(amount: number, tokenSymbol = 'ARB'): string {
  return `${
    new Intl.NumberFormat('en-US', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }).format(amount)
  } ${tokenSymbol}`;
}

export function getTimeRemaining(deadline: string): {
  total: number;
  minutes: number;
  seconds: number;
  formatted: string;
} {
  const total = Math.max(0, new Date(deadline).getTime() - Date.now());
  const minutes = Math.floor((total / 1000 / 60) % 60);
  const seconds = Math.floor((total / 1000) % 60);
  const formatted = `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  
  return { total, minutes, seconds, formatted };
}

export function getTimerColor(timeRemaining: number): string {
  if (timeRemaining < 60000) return 'text-[#F6C445]'; // Yellow for < 60s
  return 'text-[#FFF2E1]'; // Cream default
}

export function getChainName(chain: 'base' | 'arbitrum'): string {
  return chain === 'base' ? 'Base' : 'Arbitrum';
}
