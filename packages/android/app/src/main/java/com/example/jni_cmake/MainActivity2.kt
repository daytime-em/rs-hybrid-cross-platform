package com.example.jni_cmake

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Alignment.Companion.CenterHorizontally
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.example.jni_cmake.ui.theme.RustAndroidProjectTheme

class MainActivity2 : ComponentActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContent {
      RustAndroidProjectTheme {
        // A surface container using the 'background' color from the theme
        Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background) {
//          Greeting("Android")
          Content(
            isLoading = false,
            onCalculateRequested = {}
          )
        }
      }
    }
  }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
  Text(
    text = "Hello $name!",
    modifier = modifier
  )
}

@Composable
fun Content(
  isLoading: Boolean,
  onCalculateRequested: (Int) -> Unit,
  modifier: Modifier = Modifier
) {
  // Progress/Chart Area
  if (isLoading) {
    ProgressView(modifier)
  } else {
    RangeInput(
      modifier = modifier,
      onCalculateRequested = onCalculateRequested
    )
  }
}

@Composable
fun ProgressView(modifier: Modifier = Modifier) {
  Box(
    modifier = modifier
      .background(color = Color.Transparent, shape = RoundedCornerShape(12.dp))
      .border(
        width = 1.dp,
        color = Color.Gray,
        shape = RoundedCornerShape(12.dp),
      )
  ) {
    CircularProgressIndicator(
      modifier = Modifier
        .padding(all = 12.dp)
        .align(Alignment.Center)
        .size(48.dp),
      //color = Gray30
    )
  }
}

@Composable
fun RangeInput(
  modifier: Modifier = Modifier,
  initialNumber: Int? = null,
  onCalculateRequested: (Int) -> Unit,
) {
  Column(
    modifier = modifier
      .background(color = Color.Transparent, shape = RoundedCornerShape(12.dp))
      .border(
        width = 1.dp,
        color = Color.Gray,
        shape = RoundedCornerShape(12.dp),
      )
      .padding(all = 12.dp)
  ) {
    val numberInput = remember { mutableStateOf(initialNumber) }
    var calcEnabled = false
    TextField(
      value = numberInput.value?.toString() ?: "",
      placeholder = { Text("Positive Integer up to MAX_INT") },
      onValueChange = { newValue ->
        calcEnabled = try {
          val valAsInt = newValue.toInt()
          valAsInt > 0
        } catch (e: NumberFormatException) {
          false
        }
        numberInput.value = null
      },
      modifier = Modifier
        .fillMaxWidth()
    )
    Spacer(modifier = Modifier.height(12.dp))
    Button(
      onClick = {
        val inputValue = numberInput.value
        if (inputValue != null) {
          onCalculateRequested(inputValue)
        }
      },
      enabled = calcEnabled,
      modifier = Modifier
        .align(CenterHorizontally)
    ) {
      Text("Calculate!")
    }
  }
}

@Preview(showBackground = true)
@Composable
fun ProgressPreview() {
  RustAndroidProjectTheme {
    ProgressView()
  }
}

@Preview(showBackground = true)
@Composable
fun InputPreview() {
  RustAndroidProjectTheme {
    RangeInput(initialNumber = -1, onCalculateRequested = { /*nothing, is preview */ })
  }
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
  RustAndroidProjectTheme {
    Greeting("Android")
  }
}
