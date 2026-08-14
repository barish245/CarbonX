import { expect, test, describe, vi } from 'vitest';
import React from 'react';

// Mock browser-only wallet integrations
vi.mock('@stellar/freighter-api', () => ({
  default: {
    isConnected: () => Promise.resolve({ isConnected: false }),
    getAddress: () => Promise.resolve({ address: "" }),
    signTransaction: () => Promise.resolve({ signedTxXdr: "" }),
  },
  isConnected: () => Promise.resolve({ isConnected: false }),
  getAddress: () => Promise.resolve({ address: "" }),
  signTransaction: () => Promise.resolve({ signedTxXdr: "" }),
}));

vi.mock('@creit.tech/stellar-wallets-kit', () => ({
  StellarWalletsKit: {
    init: () => {},
    authModal: () => Promise.resolve({ address: "" }),
    signTransaction: () => Promise.resolve({ signedTxXdr: "" }),
  },
  Networks: {
    TESTNET: "Test SDF Network ; September 2015"
  }
}));

import { render, screen, fireEvent } from '@testing-library/react';
import Home from '../src/app/page';
import { calculateEmissions, getEsgTier } from '../src/lib/calculator';

describe('CarbonX Frontend App Tests', () => {
  test('renders the landing page initially', () => {
    render(<Home />);
    expect(screen.getByText('Carbon Markets for Every SME')).toBeDefined();
    expect(screen.getByText('Enter Terminal')).toBeDefined();
  });

  test('entering terminal switches screen to app workspace', () => {
    render(<Home />);
    const enterButton = screen.getByText('Enter Terminal');
    fireEvent.click(enterButton);

    // Should now show the active workspace sidebar element
    expect(screen.getByText('Overview Dashboard')).toBeDefined();
    expect(screen.getByText('Active Workspace')).toBeDefined();
  });

  test('clicking tabs changes visible content', () => {
    render(<Home />);
    fireEvent.click(screen.getByText('Enter Terminal'));

    // Switch to Marketplace tab
    const marketplaceBtn = screen.getByText('Marketplace');
    fireEvent.click(marketplaceBtn);

    expect(screen.getByText('Marketplace Terminal')).toBeDefined();
    expect(screen.getByText('Open Listings')).toBeDefined();
  });

  test('opens carbon footprint calculator modal', () => {
    render(<Home />);
    fireEvent.click(screen.getByText('Enter Terminal'));

    const calcBtn = screen.getByText('Calculate Footprint');
    fireEvent.click(calcBtn);

    expect(screen.getByText('SME Carbon Footprint Calculator')).toBeDefined();
    expect(screen.getByText('Offset via Marketplace')).toBeDefined();
  });

  test('calculates correct emission factors', () => {
    const res = calculateEmissions({
      electricityKwh: 10000,
      flightKm: 2000,
      fuelLiters: 500,
      serverHours: 1000
    });

    expect(res.totalTons).toBeGreaterThan(0);
    expect(res.recommendedCredits).toBeGreaterThanOrEqual(1);
    expect(res.breakdown.electricity).toBeCloseTo(4.2, 1);
  });

  test('determines correct ESG tier badges', () => {
    expect(getEsgTier(95).name).toBe('Platinum');
    expect(getEsgTier(80).name).toBe('Gold');
    expect(getEsgTier(65).name).toBe('Silver');
    expect(getEsgTier(40).name).toBe('Bronze');
  });
});
