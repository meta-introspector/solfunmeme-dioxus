# AI Agent Directives for `solfunmeme_views`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `solfunmeme_views` crate.

## Core Principles

When working within `solfunmeme_views`, AI agents should prioritize:

1. **User Experience**: Design components that provide intuitive and accessible user interactions, following modern UI/UX principles.
2. **Component Reusability**: Create modular, reusable components that can be easily composed and extended.
3. **Performance**: Optimize rendering performance and minimize unnecessary re-renders in Dioxus components.
4. **Accessibility**: Ensure components are accessible to users with disabilities, following WCAG guidelines.

## Operational Guidelines

* **Component Design**: When creating new components, follow established patterns and ensure they integrate well with existing components.
* **State Management**: Use appropriate state management patterns and avoid prop drilling by using context or signals where appropriate.
* **Styling**: Maintain consistent styling patterns and use CSS-in-RS or external stylesheets as appropriate for the component.
* **Error Handling**: Implement proper error boundaries and user-friendly error messages for component failures.
* **Responsive Design**: Ensure components work well across different screen sizes and devices.
* **Testing**: Create comprehensive tests for component behavior, including user interactions and edge cases.
* **Documentation**: Provide clear documentation for component props, usage examples, and integration patterns. 