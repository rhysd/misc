#include <iostream>
#include <string>
#include <cmath>

#define BOOST_RESULT_OF_USE_DECLTYPE
#define BOOST_SPIRIT_USE_PHOENIX_V3

#include <boost/variant/variant.hpp>
#include <boost/variant/recursive_wrapper.hpp>
#include <boost/variant/static_visitor.hpp>
#include <boost/variant/apply_visitor.hpp>
#include <boost/spirit/include/qi.hpp>
#include <boost/spirit/include/phoenix.hpp>

enum struct operators {
    add, sub, mul, div, log, exp
};

template<operators Op>
struct binary_operator;

template<operators Op>
struct unary_operator;

constexpr static
char const* to_string(operators op)
{
    switch(op) {
        case operators::add: return "+";
        case operators::sub: return "-";
        case operators::mul: return "*";
        case operators::div: return "/";
        case operators::log: return "log";
        case operators::exp: return "exp";
        default: return nullptr;
    }
}

typedef boost::variant< double
                      , boost::recursive_wrapper<binary_operator<operators::add> >
                      , boost::recursive_wrapper<binary_operator<operators::sub> >
                      , boost::recursive_wrapper<binary_operator<operators::mul> >
                      , boost::recursive_wrapper<binary_operator<operators::div> >
                      , boost::recursive_wrapper<unary_operator<operators::log> >
                      , boost::recursive_wrapper<unary_operator<operators::exp> >
                      > expression;

template<operators Op>
struct binary_operator {
    expression left;
    expression right;
    binary_operator(expression const& lhs, expression const& rhs)
        : left(lhs), right(rhs){}
};

template<operators Op>
struct unary_operator {
    expression child;
    unary_operator(expression const& rhs)
        : child(rhs){}
};

struct calculator : public boost::static_visitor<double>{
    double operator()(double const constant) const
    {
        return constant;
    }

    double operator()(binary_operator<operators::add> const& op) const
    {
        return boost::apply_visitor( calculator(), op.left )
            + boost::apply_visitor( calculator(), op.right );
    }

    double operator()(binary_operator<operators::sub> const& op) const
    {
        return boost::apply_visitor( calculator(), op.left )
            - boost::apply_visitor( calculator(), op.right );
    }

    double operator()(binary_operator<operators::mul> const& op) const
    {
        return boost::apply_visitor( calculator(), op.left )
            * boost::apply_visitor( calculator(), op.right );
    }

    double operator()(binary_operator<operators::div> const& op) const
    {
        return boost::apply_visitor( calculator(), op.left )
            / boost::apply_visitor( calculator(), op.right );
    }

    double operator()(unary_operator<operators::log> const& op) const
    {
        return std::log( boost::apply_visitor( calculator(), op.child ) );
    }

    double operator()(unary_operator<operators::exp> const& op) const
    {
        return std::exp( boost::apply_visitor( calculator(), op.child ) );
    }
};

inline double calc( expression const& expr )
{
    return boost::apply_visitor(calculator(), expr);
}

struct stringizer : public boost::static_visitor<std::string>{

    std::string operator()(double const constant) const
    {
        return std::to_string(constant);
    }

    template<operators BinaryOp>
    std::string operator()(binary_operator<BinaryOp> const& op) const
    {
        return '(' + boost::apply_visitor(stringizer(), op.left)
                   + ::to_string(BinaryOp)
                   + boost::apply_visitor(stringizer(), op.right) + ')';
    }

    template<operators UnaryOp>
    std::string operator()(unary_operator<UnaryOp> const& op) const
    {
        return to_string(UnaryOp) + ('(' + boost::apply_visitor(stringizer(), op.child) + ')');
    }

};

inline std::string stringize(expression const& expr)
{
    return boost::apply_visitor(stringizer(), expr);
}

template<operators Op>
auto make_binary_operator()
{
    return boost::phoenix::bind(
            [](auto const& lhs, auto const& rhs){
                return binary_operator<Op>(lhs, rhs);
            }, boost::spirit::_val, boost::spirit::_1 );
}

template<operators Op>
auto make_unary_operator()
{
    return boost::phoenix::bind([](auto const& e){ return unary_operator<Op>(e); }, boost::spirit::_1);
}

namespace parser_impl {
using namespace boost::spirit;

template<typename Iterator>
struct expr_grammer
    : qi::grammar<Iterator, expression(), ascii::space_type>
{
    qi::rule<Iterator, expression(), ascii::space_type> expr, term, fctr;

    expr_grammer() : expr_grammer::base_type(expr)
    {
        expr = term[_val = _1] >> *( ('+' >> term[_val = make_binary_operator<operators::add>()])
                                   | ('-' >> term[_val = make_binary_operator<operators::sub>()]) );
        term = fctr[_val = _1] >> *( ('*' >> fctr[_val = make_binary_operator<operators::mul>()])
                                   | ('/' >> fctr[_val = make_binary_operator<operators::div>()]) );
        fctr = double_[_val = _1]
             | '(' >> expr[_val = _1] >> ')'
             | "log(" >> expr[_val = make_unary_operator<operators::log>()] >> ")"
             | "exp(" >> expr[_val = make_unary_operator<operators::exp>()] >> ")";
    }
};
} // namespace hoge

// Input :  1 + 2 * (3 + 4)
// Output : (1.000000+(2.000000*(3.000000+4.000000))) = 15
int main()
{
    std::string buffer;
    while(std::getline(std::cin, buffer)){
        expression expr;
        parser_impl::expr_grammer<decltype(buffer.begin())> p;
        if( boost::spirit::qi::phrase_parse(buffer.begin(), buffer.end(), p, boost::spirit::ascii::space, expr) ){
            std::cout << stringize(expr) << " = " << calc(expr) << std::endl;
        } else {
            std::cerr << "Parse error!!\n";
        }
    }
    return 0;
}
