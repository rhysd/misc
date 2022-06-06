const PREC = {
    ADD: 1,
    MULT: 2,
    UNARY: 3,
};

module.exports = grammar({
    name: 'calc',

    extras: $ => [
        /\s/,
        $.comment,
    ],

    rules: {
        source_file: $ => repeat($._statement),

        _statement: $ => seq($._expression, /\n/),

        _expression: $ => choice(
            $.binary_expression,
            $.unary_expression,
            $.constant,
            $.paren_expression,
        ),

        binary_expression: $ => choice(
            prec.left(PREC.MULT, seq(
                field('left', ($._expression)),
                field('operator', choice('*', '/')),
                field('right', $._expression),
            )),
            prec.left(PREC.ADD, seq(
                field('left', $._expression),
                field('operator', choice('+', '-')),
                field('right', $._expression),
            )),
        ),

        unary_expression: $ => prec.left(PREC.UNARY, seq(
            field('operator', choice('+', '-')),
            field('operand', $._expression),
        )),

        constant: $ => {
            const hexLit = seq(choice('0x', '0X'), /[0-9a-fA-F]+/);
            const binLit = seq(choice('0b', '0B'), /[01]+/);

            const integer = choice('0', seq(/[1-9]/, /\d*/));
            const digits = /\d+/;
            const decimal = seq('.', digits);
            const exponent = seq(choice('e', 'E'), digits)
            const decimalLit = seq(
                integer,
                optional(decimal),
                optional(exponent),
            );

            return token(choice(
                hexLit,
                binLit,
                decimalLit,
            ));
        },

        paren_expression: $ => seq('(', $._expression, ')'),

        comment: $ => token(seq('#', /.*/)),
    },
});
