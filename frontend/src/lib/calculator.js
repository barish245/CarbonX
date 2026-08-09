// Carbon Footprint Calculator & Estimation Utility for SMEs

export const EMISSION_FACTORS = {
  electricityKwh: 0.00042,    // 0.42 kg CO2e per kWh -> ~0.00042 tCO2e
  flightKm: 0.00015,          // 0.15 kg CO2e per passenger-km -> ~0.00015 tCO2e
  fuelLiters: 0.00231,        // 2.31 kg CO2e per liter of diesel/petrol -> ~0.00231 tCO2e
  serverComputeHours: 0.00005 // 0.05 kg CO2e per cloud VM hour -> ~0.00005 tCO2e
};

export function calculateEmissions({
  electricityKwh = 0,
  flightKm = 0,
  fuelLiters = 0,
  serverHours = 0
}) {
  const electricityTons = (Number(electricityKwh) || 0) * EMISSION_FACTORS.electricityKwh;
  const flightsTons = (Number(flightKm) || 0) * EMISSION_FACTORS.flightKm;
  const fuelTons = (Number(fuelLiters) || 0) * EMISSION_FACTORS.fuelLiters;
  const serverTons = (Number(serverHours) || 0) * EMISSION_FACTORS.serverComputeHours;

  const totalTons = electricityTons + flightsTons + fuelTons + serverTons;
  const roundedTons = Math.max(1, Math.ceil(totalTons));

  return {
    breakdown: {
      electricity: Number(electricityTons.toFixed(2)),
      flights: Number(flightsTons.toFixed(2)),
      fuel: Number(fuelTons.toFixed(2)),
      cloud: Number(serverTons.toFixed(2))
    },
    totalTons: Number(totalTons.toFixed(2)),
    recommendedCredits: roundedTons,
    estimatedXlmCost: roundedTons * 0.42 // assuming ~0.42 XLM per tCO2e benchmark
  };
}

export function getEsgTier(score = 50) {
  if (score >= 90) {
    return { name: "Platinum", color: "text-cyan-300 border-cyan-500/50 bg-cyan-950/30", badge: "🏆 Platinum Level" };
  }
  if (score >= 75) {
    return { name: "Gold", color: "text-amber-300 border-amber-500/50 bg-amber-950/30", badge: "🥇 Gold Level" };
  }
  if (score >= 60) {
    return { name: "Silver", color: "text-slate-300 border-slate-400/50 bg-slate-800/40", badge: "🥈 Silver Level" };
  }
  return { name: "Bronze", color: "text-emerald-400 border-emerald-500/40 bg-emerald-950/20", badge: "🥉 Bronze Level" };
}
