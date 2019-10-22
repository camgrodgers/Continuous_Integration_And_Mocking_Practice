pipeline {
	agent any
    stages {
        stage('Build') {
            steps {
                sh 'rustup default stable'
                sh 'cargo build --release'
            }
        }
        stage('UnitTest') {
            steps {
                sh 'cargo test unit'
            }
        }
        stage('DummyTest') {
            steps {
                sh 'cargo test mock'
            }
        }
	}
}

