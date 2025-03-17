import { PREMIUM_AMOUNT } from './lightning';

declare global {
  interface Window {
    webln?: {
      enabled: boolean;
      enable: () => Promise<void>;
      sendPayment: (paymentRequest: string) => Promise<{ preimage: string }>;
    };
  }
}

export const isAlbyInstalled = (): boolean => {
  return typeof window !== 'undefined' && !!window.webln;
};

export const requestAlbyAccess = async (): Promise<void> => {
  if (!window.webln) {
    throw new Error("L'extension Alby n'est pas installée");
  }
  await window.webln.enable();
};

export const sendPaymentWithAlby = async (paymentRequest: string): Promise<{ preimage: string }> => {
  if (!window.webln) {
    throw new Error("L'extension Alby n'est pas installée");
  }
  return window.webln.sendPayment(paymentRequest);
}; 