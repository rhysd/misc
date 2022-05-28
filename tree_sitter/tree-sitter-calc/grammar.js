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

        constant: $ => token(choice(
            '0',
            seq(/[1-9]/, /\d*/),
        )),

        comment: $ => token(seq('#', /.*/)),
    },
});
