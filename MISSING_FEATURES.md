# CUI Features Wanted for cascading-ui.net

Features the tutorial website would benefit from but CUI doesn't support yet.

## String concatenation in property values

Can't compose a CSS value from a variable + literal:
```
$accent: "#5865f2";
border-left: "4px solid " $accent;  // ← syntax error
```
**Workaround**: Hardcode the full value (`border-left: "4px solid #5865f2";`).
**Impact**: Can't use theme variables in shorthand CSS properties like `border`, `box-shadow`, `background`.

## Conditional rendering / if-else

No way to conditionally render content based on a variable's value. Currently using `display` toggling:
```
let $show: "block";
section { display: $show; }
```
A proper conditional (`?if $var == "value" { ... }`) would be cleaner.

## Loops / list rendering

No way to generate repeated elements from data. Each item must be hand-written:
```
item { label { text: "Item 1"; } }
item { label { text: "Item 2"; } }
item { label { text: "Item 3"; } }
```

## Active state / CSS pseudo-classes

No `:hover`, `:active`, `:focus` pseudo-class styling. The `?mouseover`/`?mouseleave` listeners can approximate hover, but require explicit state management. Would be nice to have:
```
button {
    background: "blue";
    :hover { background: "darkblue"; }
}
```

## tooltip / image properties

These are parsed but not compiled — the Wasm codegen returns `()`. Would enable:
```
button { tooltip: "Click to submit"; }
hero { image: "logo.png"; }
```

## Form inputs

No `<input>`, `<textarea>`, `<select>` support. Can't build forms. Would need something like:
```
input {
    type: "text";
    placeholder: "Enter name";
    $value: "";  // two-way binding
}
```

## URL-based routing

Currently SPA switching is done manually with display variables. True routing would enable:
```
/home { ... }
/tutorial { ... }
/tutorial/lesson-1 { ... }
```
With browser history and direct URL access.

## Transitions between states

When `apply` swaps a class or a variable changes display, the transition is instant. CSS transitions are supported (`transition: "all 0.2s ease"`) but only for property changes on the same element, not for show/hide.

## Text content with mixed inline elements

Can't mix text with links or styled spans inline:
```
paragraph {
    text: "Read the ";
    link { text: "docs"; link: "..."; }
    text: " for more.";  // ← can't have two text properties
}
```
