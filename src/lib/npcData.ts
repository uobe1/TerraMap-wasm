// Terraria NPC 数据
// 基于 settings.js Npcs 定义

export interface NPCData {
  id: number;
  name: string;
}

export const npcData: NPCData[] = [
  { id: 17, name: "Merchant" },
  { id: 18, name: "Nurse" },
  { id: 19, name: "Arms Dealer" },
  { id: 20, name: "Dryad" },
  { id: 22, name: "Guide" },
  { id: 37, name: "Old Man" },
  { id: 38, name: "Demolitionist" },
  { id: 54, name: "Clothier" },
  { id: 107, name: "Goblin Tinkerer" },
  { id: 108, name: "Wizard" },
  { id: 124, name: "Mechanic" },
  { id: 142, name: "Santa Claus" },
  { id: 160, name: "Truffle" },
  { id: 178, name: "Steampunker" },
  { id: 207, name: "Dye Trader" },
  { id: 208, name: "Party Girl" },
  { id: 209, name: "Cyborg" },
  { id: 227, name: "Painter" },
  { id: 228, name: "Witch Doctor" },
  { id: 229, name: "Pirate" },
  { id: 353, name: "Stylist" },
  { id: 369, name: "Angler" },
  { id: 441, name: "Tax Collector" },
  { id: 550, name: "Tavernkeep" },
  { id: 588, name: "Golfer" },
  { id: 633, name: "Zoologist" },
  { id: 637, name: "Town Cat" },
  { id: 638, name: "Town Dog" },
  { id: 656, name: "Town Bunny" },
  { id: 663, name: "Princess" },
];

// 按 ID 查找 NPC
export const getNpcById = (id: number): NPCData | undefined => {
  return npcData.find(npc => npc.id === id);
};

// 按名称查找 NPC
export const getNpcByName = (name: string): NPCData | undefined => {
  return npcData.find(npc => npc.name === name);
};