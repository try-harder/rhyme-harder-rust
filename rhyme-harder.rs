use std;

#[test]
fn testLettersDontRepeat_passIfTrue()
{
	assert lettersDontRepeat("ab".chars());
}

#[test]
fn testLettersDontRepeat_failIfFalse()
{
	assert !lettersDontRepeat("aba".chars());
}

#[test]
fn testAFirstIfPresent_passIfIs()
{
	assert aIsFirstIfPresent("ab".chars());
}

#[test]
fn testAFirstIfPresent_passIfNotPresent()
{
	assert aIsFirstIfPresent("bc".chars());
}

#[test]
fn testAFirstIfPresent_failIfIsNot()
{
	assert !aIsFirstIfPresent("ba".chars());
}

#[test]
fn testZLastIfPresent_passIfIs()
{
	assert zIsLastIfPresent("az".chars());
}

#[test]
fn testZLastIfPresent_passIfNotPresent()
{
	assert zIsLastIfPresent("ab".chars());
}

#[test]
fn testZLastIfPresent_failIfIsNot()
{
	assert !zIsLastIfPresent("za".chars());
}

#[test]
fn testContainsAllLetters_succeedIfDoes()
{
	assert containsAllLetters("abcdefghijklmnopqrstuvwxyz".chars());
}

#[test]
fn testContainsAllLetters_failIfNot()
{
	assert !containsAllLetters("za".chars());
}

#[test]
fn testLettersRhyme_succeedIfDo()
{
	assert lettersRhyme('a', 'h');
}

#[test]
fn testLettersRhyme_failIfDoNot()
{
	assert !lettersRhyme('a', 'b');
}
#[test]
fn testPathIsValid_succeedIfIs()
{
	assert pathIsValid("a".chars());
	assert pathIsValid("ab".chars());
	assert pathIsValid("az".chars());
}

fn pathIsValid(path : [char]) -> bool
{
	let mut isValid = lettersDontRepeat(path)
	&& aIsFirstIfPresent(path)
	&& zIsLastIfPresent(path);
	let mut index = 0u;
	while index < path.len() - 1u
	{
		isValid &= !lettersRhyme(path[index], path[index + 1u]);
		index += 1u;
	}
	isValid
}

fn fullPathIsValid(path : [char]) -> bool
{
	pathIsValid(path) && containsAllLetters(path)
}

fn lettersDontRepeat(letters : [char]) -> bool
{
	for letter in letters
	{
		if letters.count(letter) > 1u
		{
			ret false;
		}
	}
	true
}

fn aIsFirstIfPresent(letters : [char]) -> bool
{
	!letters.contains('a') || letters[0] == 'a'
}

fn zIsLastIfPresent(letters : [char]) -> bool
{
	!letters.contains('z') || letters[letters.len() - 1u] == 'z'
}

fn containsAllLetters(letters : [char]) -> bool
{
	let allLetters = "abcdefghijklmnopqrstuvwxyz".chars();
	for letter in allLetters
	{
		if !letters.contains(letter)
		{
			ret false;
		}
	}
	true
}

fn lettersRhyme(first : char, second : char) -> bool
{
	let rhymeSets = [set("ahjk"),set("iy"),set("bcdegptvz"),set("flmnxs"),set("quw"),set("o"),set("r")];
	for rhymeSet in rhymeSets
	{
		if rhymeSet.contains(first) && rhymeSet.contains(second)
		{
			ret true;
		}
	}
	false
}
fn set(input : str) -> [char]
{
	input.chars()
}

impl str_utils for str
{
	fn chars() -> [char] {str::chars(self)}
}
impl vec_utils<T> for [T]
{
	fn count(value:T) -> uint {vec::count(self, value)}
	fn contains(value:T) -> bool {vec::contains(self, value)}
}