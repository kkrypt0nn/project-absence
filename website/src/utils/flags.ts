export const ZERO = 0;

export enum DomainFlags {
  IS_RECENT = 1 << 0,
  HAS_EXPIRED = 1 << 1,
  POSSIBLE_TAKEOVER = 1 << 2,
}

export function checkFlag(flag: number, against: number): boolean {
  return (flag & against) === against;
}
