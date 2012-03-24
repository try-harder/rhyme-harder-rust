use std;

#[test]
fn testLettersDontRepeat_passIfTrue()
{
	assert lettersDontRepeat(str::to_chars("ab"));
}

#[test]
fn testLettersDontRepeat_failIfFalse()
{
	assert !lettersDontRepeat(str::to_chars("aba"));
}

#[test]
fn testAFirstIfPresent_passIfIs()
{
	assert aIsFirstIfPresent(str::to_chars("ab"));
}

#[test]
fn testAFirstIfPresent_passIfNotPresent()
{
	assert aIsFirstIfPresent(str::to_chars("bc"));
}

#[test]
fn testAFirstIfPresent_failIfIsNot()
{
	assert !aIsFirstIfPresent(str::to_chars("ba"));
}

#[test]
fn testZLastIfPresent_passIfIs()
{
	assert zIsLastIfPresent(str::to_chars("az"));
}

#[test]
fn testZLastIfPresent_passIfNotPresent()
{
	assert zIsLastIfPresent(str::to_chars("ab"));
}

#[test]
fn testZLastIfPresent_failIfIsNot()
{
	assert !zIsLastIfPresent(str::to_chars("za"));
}

#[test]
fn testContainsAllLetters_succeedIfDoes()
{
	assert containsAllLetters(str::to_chars("abcdefghijklmnopqrstuvwxyz"));
}

#[test]
fn testContainsAllLetters_failIfNot()
{
	assert !containsAllLetters(str::to_chars("za"));
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
	assert pathIsValid(str::to_chars("a"));
	assert pathIsValid(str::to_chars("ab"));
	assert pathIsValid(str::to_chars("az"));
}

fn pathIsValid(path : [char]) -> bool
{
	let isValid = lettersDontRepeat(path)
	&& aIsFirstIfPresent(path)
	&& zIsLastIfPresent(path);
	let index = 0u;
	while index < vec::len(path) - 1u
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

fn lettersDontRepeat(letters : [const char]) -> bool
{
	for letter in letters
	{
		if vec::count(letter, letters) > 1u
		{
			ret false;
		}
	}
	true
}

fn aIsFirstIfPresent(letters : [char]) -> bool
{
	!vec::member('a', letters) || letters[0] == 'a'
}

fn zIsLastIfPresent(letters : [char]) -> bool
{
	!vec::member('z', letters) || letters[vec::len(letters) - 1u] == 'z'
}

fn containsAllLetters(letters : [char]) -> bool
{
	let allLetters = str::to_chars("abcdefghijklmnopqrstuvwxyz");
	for letter in allLetters
	{
		if !vec::member(letter, letters)
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
		if vec::member(first, rhymeSet) && vec::member(second, rhymeSet)
		{
			ret true;
		}
	}
	false
}
fn set(input : str) -> [char]
{
	str::to_chars(input)
}