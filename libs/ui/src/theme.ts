export const sizings = [
    "sm",
    "md",
    "lg",
    "xl",
] as const

export type Sizing = typeof sizings[number]

export const baseColors = [
    "base",
    "surface",
    "overlay",
    "primary",
] as const

export type BaseColor = typeof baseColors[number]

export const colors = [
    "base",
    "surface",
    "overlay",
    "primary",
    "secondary",
    "green",
    "red",
    "blue",
    "orange",
    "teal"
]

export type Color = typeof colors[number]

export const meaningfulColors = [
    "primary",
    "secondary",
    "success",
    "warning",
    "danger",
    "link",
    "muted"
] as const

export type MeaningfulColor = typeof meaningfulColors[number]
