export enum ColorVariant {
  Primary = 'primary',
  Secondary = 'secondary',
  Success = 'success',
  Danger = 'danger',
  Link = 'link',
  Disabled = 'disabled',
}

export type ModalColorVariant = Exclude<ColorVariant, ColorVariant.Disabled | ColorVariant.Link>;


export const ModalColorVariant: Readonly<Record<ModalColorVariant, ModalColorVariant>> = {
  [ColorVariant.Primary]: ColorVariant.Primary,
  [ColorVariant.Secondary]: ColorVariant.Secondary,
  [ColorVariant.Success]: ColorVariant.Success,
  [ColorVariant.Danger]: ColorVariant.Danger
}

export enum ButtonShape {
  Square = 'square',
  Rounded = 'rounded',
  Pill = 'pill',
  Circle = 'circle',
}
