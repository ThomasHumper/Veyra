# Veyra Parser — C++17

This parser handles:

- Integer and boolean literals
- Variables
- Arithmetic expressions
- Comparisons
- Variable declarations with `lum`
- `if` / `else`
- Blocks
- Functions
- Function calls
- `return`
- `print`
- Operator precedence

## `parser.cpp`

```cpp
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>
#include <vector>

enum class TokenType {
    Number,
    Identifier,

    Plus,
    Minus,
    Star,
    Slash,

    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,

    Let,
    If,
    Else,
    Fun,
    Return,
    Print,

    True,
    False,

    End
};

struct Token {
    TokenType type;
    std::string lexeme;
    int line = 0;
};

/*
 * ============================================================
 * AST
 * ============================================================
 */

struct Expr {
    virtual ~Expr() = default;
};

using ExprPtr = std::unique_ptr<Expr>;

struct NumberExpr final : Expr {
    long long value;

    explicit NumberExpr(long long value)
        : value(value) {}
};

struct BoolExpr final : Expr {
    bool value;

    explicit BoolExpr(bool value)
        : value(value) {}
};

struct VariableExpr final : Expr {
    std::string name;

    explicit VariableExpr(std::string name)
        : name(std::move(name)) {}
};

struct UnaryExpr final : Expr {
    TokenType op;
    ExprPtr right;

    UnaryExpr(TokenType op, ExprPtr right)
        : op(op), right(std::move(right)) {}
};

struct BinaryExpr final : Expr {
    ExprPtr left;
    TokenType op;
    ExprPtr right;

    BinaryExpr(
        ExprPtr left,
        TokenType op,
        ExprPtr right
    )
        : left(std::move(left)),
          op(op),
          right(std::move(right)) {}
};

struct CallExpr final : Expr {
    std::string name;
    std::vector<ExprPtr> arguments;

    CallExpr(
        std::string name,
        std::vector<ExprPtr> arguments
    )
        : name(std::move(name)),
          arguments(std::move(arguments)) {}
};


/*
 * ============================================================
 * STATEMENTS
 * ============================================================
 */

struct Stmt {
    virtual ~Stmt() = default;
};

using StmtPtr = std::unique_ptr<Stmt>;

struct ExpressionStmt final : Stmt {
    ExprPtr expression;

    explicit ExpressionStmt(ExprPtr expression)
        : expression(std::move(expression)) {}
};

struct LetStmt final : Stmt {
    std::string name;
    ExprPtr initializer;

    LetStmt(
        std::string name,
        ExprPtr initializer
    )
        : name(std::move(name)),
          initializer(std::move(initializer)) {}
};

struct PrintStmt final : Stmt {
    ExprPtr expression;

    explicit PrintStmt(ExprPtr expression)
        : expression(std::move(expression)) {}
};

struct BlockStmt final : Stmt {
    std::vector<StmtPtr> statements;

    explicit BlockStmt(std::vector<StmtPtr> statements)
        : statements(std::move(statements)) {}
};

struct IfStmt final : Stmt {
    ExprPtr condition;
    StmtPtr thenBranch;
    StmtPtr elseBranch;

    IfStmt(
        ExprPtr condition,
        StmtPtr thenBranch,
        StmtPtr elseBranch
    )
        : condition(std::move(condition)),
          thenBranch(std::move(thenBranch)),
          elseBranch(std::move(elseBranch)) {}
};

struct ReturnStmt final : Stmt {
    ExprPtr value;

    explicit ReturnStmt(ExprPtr value)
        : value(std::move(value)) {}
};

struct FunctionStmt final : Stmt {
    std::string name;
    std::vector<std::string> parameters;
    std::vector<StmtPtr> body;

    FunctionStmt(
        std::string name,
        std::vector<std::string> parameters,
        std::vector<StmtPtr> body
    )
        : name(std::move(name)),
          parameters(std::move(parameters)),
          body(std::move(body)) {}
};


/*
 * ============================================================
 * PARSER
 * ============================================================
 */

class Parser {
public:
    explicit Parser(std::vector<Token> tokens)
        : tokens_(std::move(tokens)) {}

    std::vector<StmtPtr> parse() {
        std::vector<StmtPtr> statements;

        while (!check(TokenType::End)) {
            statements.push_back(statement());
        }

        return statements;
    }

private:
    std::vector<Token> tokens_;
    std::size_t current_ = 0;

    /*
     * --------------------------------------------------------
     * Token helpers
     * --------------------------------------------------------
     */

    const Token& peek() const {
        return tokens_[current_];
    }

    const Token& previous() const {
        return tokens_[current_ - 1];
    }

    bool check(TokenType type) const {
        return peek().type == type;
    }

    const Token& advance() {
        if (!check(TokenType::End)) {
            ++current_;
        }

        return previous();
    }

    bool match(TokenType type) {
        if (!check(type)) {
            return false;
        }

        advance();
        return true;
    }

    Token consume(
        TokenType type,
        const std::string& message
    ) {
        if (check(type)) {
            return advance();
        }

        error(message);
    }

    [[noreturn]]
    void error(const std::string& message) const {
        throw std::runtime_error(
            "Parser error at line " +
            std::to_string(peek().line) +
            ": " +
            message
        );
    }

    /*
     * --------------------------------------------------------
     * Statements
     * --------------------------------------------------------
     */

    StmtPtr statement() {
        if (match(TokenType::Let)) {
            return letStatement();
        }

        if (match(TokenType::If)) {
            return ifStatement();
        }

        if (match(TokenType::Fun)) {
            return functionStatement();
        }

        if (match(TokenType::Return)) {
            return returnStatement();
        }

        if (match(TokenType::Print)) {
            return printStatement();
        }

        if (match(TokenType::LeftBrace)) {
            return std::make_unique<BlockStmt>(block());
        }

        return expressionStatement();
    }

    StmtPtr letStatement() {
        Token name = consume(
            TokenType::Identifier,
            "expected variable name after 'lum'"
        );

        consume(
            TokenType::Equal,
            "expected '=' after variable name"
        );

        ExprPtr initializer = expression();

        consume(
            TokenType::Semicolon,
            "expected ';' after variable declaration"
        );

        return std::make_unique<LetStmt>(
            name.lexeme,
            std::move(initializer)
        );
    }

    StmtPtr printStatement() {
        ExprPtr expression = expression();

        consume(
            TokenType::Semicolon,
            "expected ';' after print expression"
        );

        return std::make_unique<PrintStmt>(
            std::move(expression)
        );
    }

    StmtPtr returnStatement() {
        ExprPtr value;

        if (!check(TokenType::Semicolon)) {
            value = expression();
        }

        consume(
            TokenType::Semicolon,
            "expected ';' after return"
        );

        return std::make_unique<ReturnStmt>(
            std::move(value)
        );
    }

    StmtPtr expressionStatement() {
        ExprPtr expression = expression();

        consume(
            TokenType::Semicolon,
            "expected ';' after expression"
        );

        return std::make_unique<ExpressionStmt>(
            std::move(expression)
        );
    }

    StmtPtr ifStatement() {
        ExprPtr condition = expression();

        consume(
            TokenType::LeftBrace,
            "expected '{' after if condition"
        );

        StmtPtr thenBranch =
            std::make_unique<BlockStmt>(block());

        StmtPtr elseBranch;

        if (match(TokenType::Else)) {
            consume(
                TokenType::LeftBrace,
                "expected '{' after else"
            );

            elseBranch =
                std::make_unique<BlockStmt>(block());
        }

        return std::make_unique<IfStmt>(
            std::move(condition),
            std::move(thenBranch),
            std::move(elseBranch)
        );
    }

    StmtPtr functionStatement() {
        Token name = consume(
            TokenType::Identifier,
            "expected function name after 'fun'"
        );

        consume(
            TokenType::LeftParen,
            "expected '(' after function name"
        );

        std::vector<std::string> parameters;

        if (!check(TokenType::RightParen)) {
            do {
                Token parameter = consume(
                    TokenType::Identifier,
                    "expected parameter name"
                );

                parameters.push_back(parameter.lexeme);
            } while (match(TokenType::Comma));
        }

        consume(
            TokenType::RightParen,
            "expected ')' after parameters"
        );

        consume(
            TokenType::LeftBrace,
            "expected '{' before function body"
        );

        std::vector<StmtPtr> body = block();

        return std::make_unique<FunctionStmt>(
            name.lexeme,
            std::move(parameters),
            std::move(body)
        );
    }

    std::vector<StmtPtr> block() {
        std::vector<StmtPtr> statements;

        while (
            !check(TokenType::RightBrace) &&
            !check(TokenType::End)
        ) {
            statements.push_back(statement());
        }

        consume(
            TokenType::RightBrace,
            "expected '}' after block"
        );

        return statements;
    }

    /*
     * --------------------------------------------------------
     * Expressions
     * --------------------------------------------------------
     *
     * Precedence:
     *
     * equality
     *   comparison
     *     term
     *       factor
     *         unary
     *           primary
     */

    ExprPtr expression() {
        return equality();
    }

    ExprPtr equality() {
        ExprPtr expr = comparison();

        while (
            match(TokenType::EqualEqual) ||
            match(TokenType::BangEqual)
        ) {
            TokenType op = previous().type;

            ExprPtr right = comparison();

            expr = std::make_unique<BinaryExpr>(
                std::move(expr),
                op,
                std::move(right)
            );
        }

        return expr;
    }

    ExprPtr comparison() {
        ExprPtr expr = term();

        while (
            match(TokenType::Less) ||
            match(TokenType::LessEqual) ||
            match(TokenType::Greater) ||
            match(TokenType::GreaterEqual)
        ) {
            TokenType op = previous().type;

            ExprPtr right = term();

            expr = std::make_unique<BinaryExpr>(
                std::move(expr),
                op,
                std::move(right)
            );
        }

        return expr;
    }

    ExprPtr term() {
        ExprPtr expr = factor();

        while (
            match(TokenType::Plus) ||
            match(TokenType::Minus)
        ) {
            TokenType op = previous().type;

            ExprPtr right = factor();

            expr = std::make_unique<BinaryExpr>(
                std::move(expr),
                op,
                std::move(right)
            );
        }

        return expr;
    }

    ExprPtr factor() {
        ExprPtr expr = unary();

        while (
            match(TokenType::Star) ||
            match(TokenType::Slash)
        ) {
            TokenType op = previous().type;

            ExprPtr right = unary();

            expr = std::make_unique<BinaryExpr>(
                std::move(expr),
                op,
                std::move(right)
            );
        }

        return expr;
    }

    ExprPtr unary() {
        if (match(TokenType::Minus)) {
            TokenType op = previous().type;

            ExprPtr right = unary();

            return std::make_unique<UnaryExpr>(
                op,
                std::move(right)
            );
        }

        return primary();
    }

    ExprPtr primary() {
        if (match(TokenType::Number)) {
            long long value =
                std::stoll(previous().lexeme);

            return std::make_unique<NumberExpr>(value);
        }

        if (match(TokenType::True)) {
            return std::make_unique<BoolExpr>(true);
        }

        if (match(TokenType::False)) {
            return std::make_unique<BoolExpr>(false);
        }

        if (match(TokenType::Identifier)) {
            std::string name = previous().lexeme;

            /*
             * Identifier followed by '(' is a function call.
             */
            if (match(TokenType::LeftParen)) {
                std::vector<ExprPtr> arguments;

                if (!check(TokenType::RightParen)) {
                    do {
                        arguments.push_back(expression());
                    } while (match(TokenType::Comma));
                }

                consume(
                    TokenType::RightParen,
                    "expected ')' after arguments"
                );

                return std::make_unique<CallExpr>(
                    std::move(name),
                    std::move(arguments)
                );
            }

            return std::make_unique<VariableExpr>(
                std::move(name)
            );
        }

        if (match(TokenType::LeftParen)) {
            ExprPtr expr = expression();

            consume(
                TokenType::RightParen,
                "expected ')' after expression"
            );

            return expr;
        }

        error("expected expression");
    }
};


/*
 * ============================================================
 * Example
 * ============================================================
 *
 * The parser expects the lexer to produce tokens such as:
 *
 * lum x = sai;
 *
 * fun add(a, b) {
 *     return a + b;
 * }
 *
 * print(add(ve, tri));
 */

int main() {
    /*
     * Normally these tokens would come from the Veyra lexer.
     *
     * This example constructs them manually to demonstrate
     * the parser.
     */

    std::vector<Token> tokens = {
        {TokenType::Fun, "fun", 1},
        {TokenType::Identifier, "add", 1},
        {TokenType::LeftParen, "(", 1},
        {TokenType::Identifier, "a", 1},
        {TokenType::Comma, ",", 1},
        {TokenType::Identifier, "b", 1},
        {TokenType::RightParen, ")", 1},

        {TokenType::LeftBrace, "{", 1},

        {TokenType::Return, "return", 2},
        {TokenType::Identifier, "a", 2},
        {TokenType::Plus, "+", 2},
        {TokenType::Identifier, "b", 2},
        {TokenType::Semicolon, ";", 2},

        {TokenType::RightBrace, "}", 3},

        {TokenType::Print, "print", 4},
        {TokenType::LeftParen, "(", 4},
        {TokenType::Identifier, "add", 4},
        {TokenType::LeftParen
