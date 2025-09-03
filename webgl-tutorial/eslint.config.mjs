import eslint from '@eslint/js';
import ts from 'typescript-eslint';

export default ts.config(
    eslint.configs.recommended,
    ...ts.configs.recommendedTypeChecked,
    {
        languageOptions: {
            parserOptions: {
                projectService: true,
                project: 'tsconfig.json',
            },
        },
    },
    {
        rules: {
            'prefer-spread': 'off',
            '@typescript-eslint/explicit-member-accessibility': 'off',
            eqeqeq: 'error',
            '@typescript-eslint/explicit-function-return-type': [
                'error',
                {
                    allowExpressions: true,
                },
            ],
            '@typescript-eslint/no-floating-promises': 'error',
            '@typescript-eslint/no-unnecessary-type-arguments': 'error',
            '@typescript-eslint/no-empty-interface': 'error',
            '@typescript-eslint/restrict-plus-operands': 'error',
            '@typescript-eslint/no-extra-non-null-assertion': 'error',
            '@typescript-eslint/prefer-nullish-coalescing': 'error',
            '@typescript-eslint/prefer-optional-chain': 'error',
            '@typescript-eslint/prefer-includes': 'error',
            '@typescript-eslint/prefer-for-of': 'error',
            '@typescript-eslint/prefer-string-starts-ends-with': 'error',
            '@typescript-eslint/prefer-ts-expect-error': 'error',
            '@typescript-eslint/no-non-null-asserted-optional-chain': 'error',
            '@typescript-eslint/await-thenable': 'error',
            '@typescript-eslint/no-unnecessary-boolean-literal-compare': 'error',
            '@typescript-eslint/ban-ts-comment': [
                'error',
                {
                    'ts-ignore': true,
                    'ts-nocheck': true,
                },
            ],
            '@typescript-eslint/naming-convention': [
                'error',
                {
                    selector: 'default',
                    format: ['camelCase', 'PascalCase', 'UPPER_CASE'],
                    leadingUnderscore: 'allow',
                },
            ],
            'no-unused-vars': 'off',
            '@typescript-eslint/no-unused-vars': 'error',
            '@typescript-eslint/non-nullable-type-assertion-style': 'error',
            'no-return-await': 'off',
            '@typescript-eslint/return-await': ['error', 'in-try-catch'],
            '@typescript-eslint/no-invalid-void-type': 'error',
            '@typescript-eslint/prefer-as-const': 'error',
            '@typescript-eslint/consistent-indexed-object-style': 'error',
            '@typescript-eslint/no-base-to-string': 'error',
            '@typescript-eslint/switch-exhaustiveness-check': ['error', { considerDefaultExhaustiveForUnions: true }],
        },
    },
    {
        files: ['scripts/*.ts'],
        rules: {
            'n/no-sync': 'off',
        },
    },
);
